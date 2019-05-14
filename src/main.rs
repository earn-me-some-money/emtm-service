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
            // Route URL-handlers
            // Common API Routers
            .service(web::resource("/").route(web::get().to(router::index)))
            .service(web::resource("/cow_logup").route(web::post().to(router::cow_logup)))
            .service(web::resource("/student_logup").route(web::post().to(router::stu_logup)))
            .service(web::resource("/login").route(web::post().to(router::login)))
            .service(web::resource("/release_task").route(web::post().to(router::release_task)))
            .service(web::resource("/check_task").route(web::post().to(router::check_task)))
            .service(web::resource("/recharge").route(web::post().to(router::recharge)))
            // Student Specific API Routers
            .service(web::resource("/create_group").route(web::post().to(router::create_group)))
            .service(web::resource("/join_group").route(web::post().to(router::join_group)))
            .service(web::resource("/add_friend").route(web::post().to(router::add_friend)))
            .service(web::resource("/receive_task").route(web::post().to(router::receive_task)))
            .service(web::resource("/submit_task").route(web::post().to(router::submit_task)))
            .service(web::resource("/withdraw").route(web::post().to(router::withdraw)))
    })
    .bind("localhost:8080")?
    .workers(num_cpus::get())
    .run()
}
