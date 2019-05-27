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

pub fn cow_logup(data: web::Json<json_objs::CowLogupObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the wechat authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    controller::logup_cow(
        &data.username,
        &data.userid,
        &data.phone,
        &data.email,
        &data.infos,
        &data.organization,
    )
}

pub fn stu_logup(data: web::Json<json_objs::StuLogupObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the wechat authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    controller::logup_student(data)
}

pub fn login(data: web::Json<json_objs::LoginObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the wechat authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    if data.login_mode {
        return controller::login(&data.userid, true);
    } else {
        controller::login(&data.userid, false)
    }
}

pub fn release_task(data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    controller::release_task(data)
}

pub fn check_task(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    controller::check_task(data)
}

pub fn recharge(data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    controller::recharge(data)
}

pub fn search_mission(data: web::Json<json_objs::MissionSearchObj>) -> HttpResponse {
    controller::search_mission(data)
}

// Student Specific - APIs
pub fn create_group(data: web::Json<json_objs::CreateGroupObj>) -> HttpResponse {
    controller::create_group(data)
}

pub fn join_group(data: web::Json<json_objs::JoinGroupObj>) -> HttpResponse {
    controller::join_group(data)
}

pub fn add_friend(data: web::Json<json_objs::AddFriendObj>) -> HttpResponse {
    controller::add_friend(data)
}

pub fn receive_task(data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    controller::receive_task(data)
}

pub fn submit_task(data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    controller::submit_task(data)
}

pub fn check_credit(data: web::Json<json_objs::CheckCreditObj>) -> HttpResponse {
    controller::check_credit(data)
}

pub fn withdraw(data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    controller::withdraw(data)
}
