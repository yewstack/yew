use actix::*;
#[actix_rt::test]
async fn test_server_echoes_messages() {
    let mut app = actix_web::test::start(|| {
        let server = crate::server::DevToolsServer::default().start();
        actix_web::App::new()
            .data(server)
            .service(actix_web::web::resource("/ws").to(crate::server::websocket_route))
    });
    let ws_conn_browser = app.ws_at("/ws").await.unwrap();
    let ws_conn_extension = app.ws_at("/ws").await.unwrap();
}
