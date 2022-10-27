use std::net::Ipv4Addr;
use std::env;

use actix_web::{get, web, App, HttpServer, Result, Responder};

#[get("/api/{name}")]
async fn greet(name: web::Path<String>) -> Result<impl Responder> {
    let res = format!("Hello {name} from Azure Functions + Rust (Actix-Web)");
    Ok(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new()
            .route("/api/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await

}
