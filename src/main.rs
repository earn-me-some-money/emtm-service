extern crate emtm;
extern crate env_logger;
extern crate json;

use actix_web::{middleware, web, App, HttpServer};
use emtm::route::router;

extern crate num_cpus;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(router::index)))
            .service(web::resource("/logup").route(web::post().to(router::logup)))
    })
    .bind("localhost:8080")?
    .workers(num_cpus::get())
    .run()
}
