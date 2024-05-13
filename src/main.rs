use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder { // return something that implements the Responder trait
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()//.finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new( || { // establish a new connection with clients, handles transport level concerns (i.e. TCP socket?)
        App::new() // contains all application logic
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))// request is only passed to handler f HTTP method is GET
            .route("/{name}", web::get().to(greet))
    })
            .bind("127.0.0.1:8000")?
            .run()
            .await
}