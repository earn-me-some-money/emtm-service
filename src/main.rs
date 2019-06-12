use actix_web::{middleware, web, App, HttpServer};
use emtm_web::route::router;

fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed_custom_env("EMTM_LOG").unwrap();

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    emtm_db::search::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Route URL-handlers
            // Common API Routers
            .service(web::resource("/").route(web::get().to(router::index)))
            .service(web::resource("/get_wechatid").route(web::post().to(router::get_wechatid)))
            .service(web::resource("/cow_logup").route(web::post().to(router::cow_logup)))
            .service(web::resource("/student_logup").route(web::post().to(router::stu_logup)))
            .service(web::resource("/login").route(web::post().to(router::login)))
            .service(web::resource("/release_task").route(web::post().to(router::release_task)))
            .service(web::resource("/check_task").route(web::post().to(router::check_task)))
            .service(web::resource("/recharge").route(web::post().to(router::recharge)))
            .service(web::resource("/user_verify").route(web::post().to(router::user_verify)))
            // Mission Search
            .service(web::resource("/search_mission").route(web::post().to(router::search_mission)))
            // Student Specific API Routers
            .service(web::resource("/create_group").route(web::post().to(router::create_group)))
            .service(web::resource("/join_group").route(web::post().to(router::join_group)))
            .service(web::resource("/add_friend").route(web::post().to(router::add_friend)))
            .service(web::resource("/receive_task").route(web::post().to(router::receive_task)))
            .service(web::resource("/submit_task").route(web::post().to(router::submit_task)))
            .service(web::resource("/withdraw").route(web::post().to(router::withdraw)))
    })
    .bind("127.0.0.1:6789")?
    .workers(num_cpus::get())
    .run()
}
