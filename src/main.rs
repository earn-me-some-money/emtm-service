extern crate env_logger;
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::IntoFuture;

extern crate num_cpus;

#[get("/")]
fn index(req: HttpRequest) -> &'static str {
    "Hello, world!"
}

fn index_async(req: HttpRequest) -> impl IntoFuture<Item = &'static str, Error = Error> {
    println!("REQ: {:?}", req);
    Ok("Hello async request!\r\n")
}

#[get("/{name}")]
fn hello_name(req: HttpRequest, name: web::Path<String>) -> String {
    format!("Hello, {}!\r\n", name)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(web::resource("/index").to(|| "Emtm Index Page~\r\n"))
            .service(hello_name)
            .service(
                web::resource("/async/")
                    .wrap(middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
                    .route(web::get().to_async(index_async)),
            )
    })
    .bind("localhost:8080")?
    .workers(num_cpus::get())
    .run()
}
