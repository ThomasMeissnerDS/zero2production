use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new( || { // establish a new connection with clients, handles transport level concerns (i.e. TCP socket?)
        App::new() // contains all application logic
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}