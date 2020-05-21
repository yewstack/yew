use actix::prelude::*;
use actix_web_actors::ws;
struct MsgWs;

impl Actor for MsgWs {
    type Context = ws::WebSocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MsgWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}
