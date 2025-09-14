use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("world");
//     format!("hello {}!", name)
// }
// async fn health_check(req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok()
// }

// #[tokio::main] // use tokio runtime
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/hello/{name}", web::get().to(greet))
//             .route("/hello", web::get().to(greet))
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


use emailnewsletter :: run;

#[tokio :: main]
async fn main () -> std :: io :: Result<()> {
    run().await
}