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
            .service(web::resource("/wechatid").route(web::get().to(router::get_wechatid)))
            // User Log APIs
            .service(web::resource("/logup/cow").route(web::post().to(router::cow_logup)))
            .service(web::resource("/logup/stu").route(web::post().to(router::stu_logup)))
            .service(web::resource("/login").route(web::post().to(router::login)))
            // Get Tasks APIs
            .service(web::resource("/task/type").route(web::get().to(router::get_tasks)))
            .service(web::resource("/task/range").route(web::get().to(router::get_tasks_by_range)))
            // Release Task APIs
            .service(web::resource("/task/release").route(web::post().to(router::release_task)))
            .service(
                web::resource("/task/release-question")
                    .route(web::post().to(router::release_task_question)),
            )
            .service(
                web::resource("/task/release-transaction")
                    .route(web::post().to(router::release_task_transaction)),
            )
            .service(
                web::resource("/task/release-errand")
                    .route(web::post().to(router::release_task_errand)),
            )
            // Check Task APIs
            .service(web::resource("/task/specific").route(web::get().to(router::check_task)))
            .service(
                web::resource("/task/self-receive")
                    .route(web::get().to(router::check_task_self_receive)),
            )
            .service(
                web::resource("/task/self-release")
                    .route(web::get().to(router::check_task_self_release)),
            )
            .service(
                web::resource("/task/question-naire")
                    .route(web::get().to(router::check_question_naire)),
            )
            .service(
                web::resource("/task/transaction").route(web::get().to(router::check_transaction)),
            )
            .service(web::resource("/task/errand").route(web::get().to(router::check_errand)))
            // User Verify
            .service(web::resource("/user/verify").route(web::post().to(router::user_verify)))
            // Mission Search
            .service(web::resource("/task/search").route(web::get().to(router::search_mission)))
            // Receive Tasks API
            .service(web::resource("/task/receive").route(web::post().to(router::receive_task)))
            // Submit Tasks APIs
            .service(web::resource("/task/submit").route(web::post().to(router::submit_task)))
            .service(
                web::resource("/task/submit-stu").route(web::post().to(router::submit_task_stu)),
            )
            // User balance management
            .service(web::resource("/balance/recharge").route(web::post().to(router::recharge)))
            .service(web::resource("/balance/withdraw").route(web::post().to(router::withdraw)))
            .service(web::resource("/balance").route(web::get().to(router::get_balance)))
            .service(web::resource("/credit").route(web::get().to(router::check_credit)))
            // User Infos management
            .service(web::resource("/info/cow").route(web::get().to(router::get_cow_info)))
            .service(web::resource("/info/cow/edit").route(web::post().to(router::edit_cow_info)))
            .service(web::resource("/info/stu").route(web::get().to(router::get_stu_info)))
            .service(web::resource("/info/stu/edit").route(web::post().to(router::edit_stu_info)))
    })
    .bind("0.0.0.0:6789")?
    .workers(num_cpus::get())
    .run()
}
