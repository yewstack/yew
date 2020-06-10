use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use rand::Rng;

use std::collections::{HashMap, HashSet};

const HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
const CLIENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

pub(crate) async fn websocket_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<DevToolsServer>>,
) -> impl actix_web::Responder {
    ws::start(
        DevToolsSession {
            id: 0,
            hb: std::time::Instant::now(),
            role: 0, // browser by default,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// A connection between the client and the server.
struct DevToolsSession {
    id: usize,
    hb: std::time::Instant,
    role: i32,
    addr: Addr<DevToolsServer>,
}

/// Handles a message sent by the client to the server.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DevToolsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = std::time::Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = std::time::Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(3, '/').collect();
                    if v.len() <= 0 {
                        return;
                    }
                    if let "specify" = v[1] {
                        self.addr
                            .send(SpecifyRole {
                                id: self.id,
                                role: match v[2].parse::<i32>() {
                                    Ok(p) => p,
                                    Err(_) => {
                                        return;
                                    }
                                },
                            })
                            .into_actor(self)
                            .then(|res, act, ctx| {
                                match res {
                                    Ok(r) => {
                                        act.role = r;
                                    }
                                    Err(_) => {
                                        ctx.stop();
                                    }
                                };
                                fut::ready(())
                            })
                            .wait(ctx);
                    }
                } else {
                    match self.role {
                        0 => {
                            self.addr
                                .send(ExtensionMessage {
                                    msg: text,
                                    id: self.id,
                                })
                                .into_actor(self)
                                .then(|_res, _act, _ctx| fut::ready(()))
                                .wait(ctx);
                        }
                        1 => {
                            self.addr
                                .send(BrowserMessage {
                                    msg: text,
                                    id: self.id,
                                })
                                .into_actor(self)
                                .then(|_res, _act, _ctx| fut::ready(()))
                                .wait(ctx);
                        }
                        _ => {}
                    }
                }
            }
            // ignore any of these types of message
            ws::Message::Binary(_) => {}
            ws::Message::Continuation(_) => {}
            ws::Message::Close(_) => {}
            ws::Message::Nop => {}
        }
    }
}

/// Handles starting/stopping of connections from clients to the server
impl actix::Actor for DevToolsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Sends a message to a client.
impl Handler<Message> for DevToolsSession {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl DevToolsSession {
    /// Pings the client regularly.
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if std::time::Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
/// A message to be sent to the client.
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
/// Handles a user connecting to the server.
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Handles a user disconnecting from the server.
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
/// A message to be sent to the Yew client app.
pub struct BrowserMessage {
    pub id: usize,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
/// A message to be sent to the Yew browser extension.
pub struct ExtensionMessage {
    pub id: usize,
    pub msg: String,
}

#[derive(Message)]
#[rtype(i32)]
/// Sent by the client to specify whether they are a browser extension or Yew app.
pub struct SpecifyRole {
    pub id: usize,
    pub role: i32,
}

/// Coordinates extensions and Yew apps.
pub struct DevToolsServer {
    sessions: HashMap<usize, Recipient<Message>>,
    browsers: HashSet<usize>,
    extensions: HashSet<usize>,
    rng: rand::rngs::ThreadRng,
}

impl actix::Actor for DevToolsServer {
    type Context = actix::Context<Self>;
}

impl Default for DevToolsServer {
    fn default() -> DevToolsServer {
        let sessions = HashMap::new();
        let browsers = HashSet::new();
        let extensions = HashSet::new();
        DevToolsServer {
            sessions,
            browsers,
            extensions,
            rng: rand::thread_rng(),
        }
    }
}

impl DevToolsServer {
    /// Sends a message to all the Yew apps.
    fn send_browser_message(&self, message: &str) {
        for id in self.browsers.iter() {
            if let Some(browser_sess) = self.sessions.get(id) {
                match browser_sess.do_send(Message(message.to_owned())) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
        }
    }
    /// Sends a message to all the Yew DevTools extensions.
    fn send_extension_message(&self, message: &str) {
        for id in self.extensions.iter() {
            if let Some(extension_sess) = self.sessions.get(id) {
                match extension_sess.do_send(Message(message.to_owned())) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
        }
    }
}

impl Handler<Connect> for DevToolsServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let sess_id = self.rng.gen::<usize>();
        self.sessions.insert(sess_id, msg.addr);
        sess_id
    }
}

impl Handler<Disconnect> for DevToolsServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        if self.sessions.remove(&msg.id).is_some() {
            self.browsers.remove(&msg.id);
            self.extensions.remove(&msg.id);
        }
    }
}

impl Handler<BrowserMessage> for DevToolsServer {
    type Result = ();
    fn handle(&mut self, msg: BrowserMessage, _: &mut Self::Context) {
        self.send_browser_message(&msg.msg);
    }
}

impl Handler<ExtensionMessage> for DevToolsServer {
    type Result = ();
    fn handle(&mut self, msg: ExtensionMessage, _: &mut Self::Context) {
        self.send_extension_message(&msg.msg);
    }
}

impl Handler<SpecifyRole> for DevToolsServer {
    type Result = i32;
    fn handle(&mut self, msg: SpecifyRole, _: &mut Self::Context) -> i32 {
        match msg.role {
            0 => {
                if self.extensions.get(&msg.id).is_none() {
                    self.browsers.insert(msg.id);
                }
            }
            1 => {
                if self.browsers.get(&msg.id).is_none() {
                    self.extensions.insert(msg.id);
                }
            }
            _ => {}
        };
        msg.role
    }
}
