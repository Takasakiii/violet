pub mod dtos;

use std::io::Result;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, web};

use crate::channels::GerChannels;



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
            .service(get_event_from_app)
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}

#[post("/api/app/{id_app}/events")]
async fn get_event_from_app(path: web::Path<(u64,)>, content: web::Json<dtos::EventTrackerReceive>) -> HttpResponse {
    let mut content = content.0;
    let mut ret = HttpResponse::Ok().finish();
    content.app_id = Some(path.0.0);
    GerChannels::get(|g|{
        let result = g.get_channel("send_app_event", |c|{
            c.send_data(content.clone())
                .ok();
        });

        match result {
            Ok(_) => {
                ret = HttpResponse::Created()
                    .json(content.clone());
            },
            Err(why) => {
                ret = HttpResponse::BadGateway()
                    .body(why);
            }
        }

    });
    ret
}
