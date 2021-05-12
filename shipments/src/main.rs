use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/shipments", web::get().to(index)),
        )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
