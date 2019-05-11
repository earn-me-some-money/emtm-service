/*
* Emtm-Controller Modules
*/
extern crate json;

use actix_web::{web, HttpResponse};

use crate::control::json_objs;

pub fn index() -> HttpResponse {
    let object = json_objs::OriginObj {
        code: true,
        errMessage: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    HttpResponse::Ok().json(object)
}

pub fn logup(data: web::Json<json_objs::LogupObj>) -> HttpResponse {
    // Read Request Body into Json
    println!("{:?}", &data.wechat_ok);
    HttpResponse::Ok().json(data.0)
}
