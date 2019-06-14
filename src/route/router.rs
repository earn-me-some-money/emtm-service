/*
* Router Mount APIs
*/

use actix_web::{web, HttpRequest, HttpResponse};

use crate::control::group_control;
use crate::control::json_objs;
use crate::control::log_control;
use crate::control::main_control;
use crate::control::task_control;

// Common APIs
pub fn index(_req: HttpRequest) -> HttpResponse {
    main_control::index()
}

pub fn cow_logup(data: web::Json<json_objs::CowLogupObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the User-Authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    log_control::logup_cow(
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
        err_message: "Please make sure user pass the User-Authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    log_control::logup_student(data)
}

pub fn login(data: web::Json<json_objs::LoginObj>) -> HttpResponse {
    let wechat_fail = json_objs::OriginObj {
        code: false,
        err_message: "Please make sure user pass the Wechat-Authenitication!".to_string(),
    };

    // Do WeChat-Authe Check
    if !data.wechat_ok {
        return HttpResponse::Ok().json(wechat_fail);
    }

    log_control::login(&data.userid)
}

pub fn release_task(data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    task_control::release_task(data)
}

pub fn check_task(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    task_control::check_task(data)
}

pub fn recharge(data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    main_control::recharge(data)
}

pub fn search_mission(data: web::Json<json_objs::MissionSearchObj>) -> HttpResponse {
    task_control::search_mission(data)
}

// Student Specific - APIs
pub fn create_group(data: web::Json<json_objs::CreateGroupObj>) -> HttpResponse {
    group_control::create_group(data)
}

pub fn join_group(data: web::Json<json_objs::JoinGroupObj>) -> HttpResponse {
    group_control::join_group(data)
}

pub fn add_friend(data: web::Json<json_objs::AddFriendObj>) -> HttpResponse {
    group_control::add_friend(data)
}

pub fn receive_task(data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    task_control::receive_task(data)
}

pub fn submit_task(data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    task_control::submit_task(data)
}

pub fn check_credit(data: web::Json<json_objs::CheckCreditObj>) -> HttpResponse {
    main_control::check_credit(data)
}

pub fn withdraw(data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    main_control::withdraw(data)
}

pub fn user_verify(
    data: web::Json<json_objs::VerifyInfo>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    main_control::verify(data)
}

use futures::Future;
pub fn get_wechatid(
    data: web::Json<json_objs::GetWechatIdObj>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    main_control::get_wechatid(data)
}
