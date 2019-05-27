/*
* Emtm-Controller Modules -- Group Control
*/
extern crate json;

use actix_web::{web, HttpResponse};

use crate::control::json_objs;

// Student Friend && Group Manage Function Methods

pub fn create_group(_data: web::Json<json_objs::CreateGroupObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn join_group(_data: web::Json<json_objs::JoinGroupObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn add_friend(_data: web::Json<json_objs::AddFriendObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}
