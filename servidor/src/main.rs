use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let html_content = fs::read_to_string("statics/index.html")
        .unwrap_or_else(|_| String::from("Failed to read HTML file"));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[post("/create")]
async fn create_item(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Item created")
}

#[put("/update")]
async fn update_item(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Item updated")
}

#[delete("/delete")]
async fn delete_item(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Item deleted")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurar el logger para escribir en un archivo
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("log/output.log")
        .unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_item)
            .service(update_item)
            .service(delete_item)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}