use actix_files::NamedFile;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

// You will need to change this if you use this as a template for your application.
const ASSETS_DIR: &str = "../../../target/deploy";

#[get("/api")]
async fn api_404() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[get("/api/{unconfigured_routes:.*}")]
async fn api_404_unconfigured() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[get("/api/hello/{name}")]
async fn api_hello(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(name.into_inner())
}

async fn serve_index_html() -> Result<NamedFile, Error> {
    const INDEX_HTML: &str = "index.html";
    let index_file = format!("{}/{}", ASSETS_DIR, INDEX_HTML);

    Ok(NamedFile::open(index_file)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let localhost: &str = "0.0.0.0";
    let port: u16 = 8000;
    let addr = (localhost, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(api_404)
            .service(api_hello)
            // Important this comes last so all configured api routes will match
            // before this catch all
            .service(api_404_unconfigured)
            .service(actix_files::Files::new("/", ASSETS_DIR).index_file("index.html"))
            .default_service(web::get().to(serve_index_html))
    })
    .bind(addr)?
    .workers(1)
    .run()
    .await
}
