/*
* Emtm-Controller Modules
*/
extern crate json;

use actix_web::HttpResponse;

use crate::control::json_objs;

pub fn index() -> HttpResponse {
    let object = OriginObj {
        code: true,
        errMessage: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    HttpResponse::Ok().json(object)
}

pub fn logup(data: web::Json<LogupObj>) -> HttpResponse {
    // Read Request Body into Json
    println!("{:?}", &data.wechat_ok);
    HttpResponse::Ok().json(data.0)
}
