use std::io::Result;
use actix_web::{App, HttpResponse, HttpServer, get, middleware};

#[actix_web::main]
pub async fn start_web_server() {
    actix_start()
        .await
        .expect("Não foi possivel ligar o servidor web.");
}

async fn actix_start() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .body("vulcan gay")
}
