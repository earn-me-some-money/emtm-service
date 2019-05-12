/*
* Router Mount APIs
*/
use actix_web::{web, HttpRequest, HttpResponse};

use crate::control::controller;
use crate::control::json_objs;

// Common APIs
pub fn index(_req: HttpRequest) -> HttpResponse {
    controller::index()
}

pub fn logup(_data: web::Json<json_objs::LogupObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the wechat authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !_data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    if _data.logup_mode {
        return controller::logup_student(_data);
    } else {
        controller::logup_cow(&_data.username, &_data.email, &_data.organization)
    }
}

pub fn login(_data: web::Json<json_objs::LoginObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the wechat authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !_data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    if _data.login_mode {
        return controller::login_student(&_data.username);
    } else {
        controller::login_student(&_data.username)
    }
}

pub fn release_task(_data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    controller::release_task(_data)
}

pub fn check_task(_data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    controller::check_task(_data)
}

pub fn recharge(_data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    controller::recharge(_data)
}

// Student Specific - APIs
pub fn create_group(_data: web::Json<json_objs::CreateGroupObj>) -> HttpResponse {
    controller::create_group(_data)
}

pub fn join_group(_data: web::Json<json_objs::JoinGroupObj>) -> HttpResponse {
    controller::join_group(_data)
}

pub fn add_friend(_data: web::Json<json_objs::AddFriendObj>) -> HttpResponse {
    controller::add_friend(_data)
}

pub fn receive_task(_data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    controller::receive_task(_data)
}

pub fn submit_task(_data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    controller::submit_task(_data)
}

pub fn check_credit(_data: web::Json<json_objs::CheckCreditObj>) -> HttpResponse {
    controller::check_credit(_data)
}

pub fn withdraw(_data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    controller::withdraw(_data)
}
