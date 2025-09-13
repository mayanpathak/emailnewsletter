





pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello/{name}", web::get().to(greet))
            .route("/hello", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}