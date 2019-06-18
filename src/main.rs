use actix_web::{middleware, web, App, HttpServer};
use emtm_web::route::router;

fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
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
            // User Log APIs
            .service(web::resource("/cow_logup").route(web::post().to(router::cow_logup)))
            .service(web::resource("/student_logup").route(web::post().to(router::stu_logup)))
            .service(web::resource("/login").route(web::post().to(router::login)))
            // Get Tasks APIs
            .service(web::resource("/get_tasks").route(web::post().to(router::get_tasks)))
            // Release Task APIs
            .service(web::resource("/release_task").route(web::post().to(router::release_task)))
            .service(
                web::resource("/release_task_question")
                    .route(web::post().to(router::release_task_question)),
            )
            .service(
                web::resource("/release_task_transaction")
                    .route(web::post().to(router::release_task_transaction)),
            )
            .service(
                web::resource("/release_task_errand")
                    .route(web::post().to(router::release_task_errand)),
            )
            // Check Task APIs
            .service(web::resource("/check_task").route(web::post().to(router::check_task)))
            .service(
                web::resource("/check_task_self_receive")
                    .route(web::post().to(router::check_task_self_receive)),
            )
            .service(
                web::resource("/check_task_self_release")
                    .route(web::post().to(router::check_task_self_release)),
            )
            .service(
                web::resource("/check_question_naire")
                    .route(web::post().to(router::check_question_naire)),
            )
            // User Verify
            .service(web::resource("/user_verify").route(web::post().to(router::user_verify)))
            // Mission Search
            .service(web::resource("/search_mission").route(web::post().to(router::search_mission)))
            // Student Specific API Routers
            .service(web::resource("/create_group").route(web::post().to(router::create_group)))
            .service(web::resource("/join_group").route(web::post().to(router::join_group)))
            .service(web::resource("/add_friend").route(web::post().to(router::add_friend)))
            // Receive Tasks API
            .service(web::resource("/receive_task").route(web::post().to(router::receive_task)))
            // Submit Tasks APIs
            .service(
                web::resource("/submit_task_cow").route(web::post().to(router::submit_task_cow)),
            )
            .service(
                web::resource("/submit_task_stu").route(web::post().to(router::submit_task_stu)),
            )
            // User balance management
            .service(web::resource("/recharge").route(web::post().to(router::recharge)))
            .service(web::resource("/withdraw").route(web::post().to(router::withdraw)))
            .service(web::resource("/get_balance").route(web::post().to(router::get_balance)))
            .service(web::resource("/check_credit").route(web::post().to(router::check_credit)))
            // User Infos management
            .service(web::resource("/get_cow_info").route(web::post().to(router::get_cow_info)))
            .service(web::resource("/edit_cow_info").route(web::post().to(router::edit_cow_info)))
            .service(web::resource("/get_stu_info").route(web::post().to(router::get_stu_info)))
            .service(web::resource("/edit_stu_info").route(web::post().to(router::edit_stu_info)))
    })
    .bind("127.0.0.1:6789")?
    .workers(num_cpus::get())
    .run()
}
