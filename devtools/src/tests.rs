use actix::*;
use actix_web::web::Bytes;
use futures::{SinkExt, StreamExt};

#[actix_rt::test]
async fn test_server_echoes_messages() {
    let mut app = actix_web::test::start(|| {
        let server = crate::server::DevToolsServer::default().start();
        actix_web::App::new()
            .data(server)
            .service(actix_web::web::resource("/ws").to(crate::server::websocket_route))
    });
    let mut browser_framed = app.ws_at("/ws").await.unwrap();
    browser_framed.send(actix_web_actors::ws::Message::Text("/specify/0".into()));
    let mut extension_framed = app.ws_at("/ws").await.unwrap();
    extension_framed
        .send(actix_web_actors::ws::Message::Text("/specify/1".into()))
        .await;
    browser_framed
        .send(actix_web_actors::ws::Message::Text(
            "{\"message\": \"hello world!\"}".into(),
        ))
        .await;
    assert_eq!(
        extension_framed.next().await.unwrap().unwrap(),
        actix_web_actors::ws::Frame::Text(Bytes::from_static(b"{\"message\": \"hello world!\"}"))
    );
}
