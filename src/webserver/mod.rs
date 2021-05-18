pub mod dtos;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, post, web};

use crate::{channels::GerChannels, mysql_db::{AppTable, ReportsTable}};


#[actix_web::main]
pub async fn start_web_server() {
    actix_start()
        .await
        .expect("Não foi possivel ligar o servidor web.");
}

async fn actix_start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_event_from_app)
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}

#[post("/api/apps/{id_app}/events")]
async fn get_event_from_app(path: web::Path<(u64,)>, content: web::Json<dtos::EventTrackerReceive>, req: HttpRequest) -> HttpResponse {
    let mut result = HttpResponse::InternalServerError()
        .finish();

    if let Err(why) =  event_handler(path, content, req, |data| {
        result = HttpResponse::Created()
            .json(data);
    }) {
        result = HttpResponse::BadRequest()
            .json(dtos::ErrPayload{
                message: format!("{:?}", why)
            });
    }

    result
}

fn event_handler<F>(path: web::Path<(u64,)>, content: web::Json<dtos::EventTrackerReceive>, req: HttpRequest, mut callback: F) -> Result<(), crate::GenericError>
where
    F: FnMut(ReportsTable)
{
    let token = get_token(&req)
        .ok_or_else(|| "Problemas ao verificar o token".to_string())?;

    let app_data_finded = AppTable::get(path.0.0)
        .ok_or_else(|| "Aplicação não localizada.".to_string())?;

    if app_data_finded.token_app.ne(&token) {
        return Err("Token invalido para essa aplicação.".into());
    }

    GerChannels::get(|g| {
        g.get_channel("send_app_event", |c| {
            c.send_data(content.clone())
        })
    })?;

    let report = ReportsTable::insert(content.severity.into(), &content.title, &content.message, &content.stacktrace, app_data_finded.id)?;
    callback(report);

    Ok(())
}


fn get_token (req: &HttpRequest) -> Option<String> {
    req
        .headers()
        .get("Authorization")?
        .to_str()
        .ok()
        .map(|e| e.to_string())
}
