use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Deserialize)]
struct InputData {
    token: String,
    encrypted_user_id: String,
    mobile_number: String,
}

#[derive(Serialize)]
struct OutputData {
    formatted: String,
}

#[post("/format")]
async fn format_data(data: web::Json<InputData>) -> impl Responder {
    let formatted = format!(
        " {}  userid  {}  user  {}",
        data.token, data.encrypted_user_id, data.mobile_number
    );

    let response = OutputData { formatted };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at: http://127.0.0.1:8080/format");
    HttpServer::new(|| {
        App::new()
         .wrap(Cors::permissive())
            .service(format_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
