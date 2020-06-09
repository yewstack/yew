extern crate actix;
#[macro_use]
extern crate actix_web;
extern crate structopt;

#[cfg(test)]
mod tests;

use structopt::StructOpt;

mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "dev")]
struct Opt {
    #[structopt(short, long, default_value = "8080")]
    port: u32,
    #[structopt(short, long, default_value = "localhost")]
    host: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    actix_web::HttpServer::new(|| actix_web::App::new())
        .bind(format!("{}:{}", opt.host, opt.port))?
        .run()
        .await
}
