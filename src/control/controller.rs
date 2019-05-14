/*
* Emtm-Controller Modules
*/
extern crate emtm_db;
extern crate json;
extern crate regex;

use actix_web::{web, HttpResponse};
use regex::Regex;

use crate::control::json_objs;

// Basic Function Methods

pub fn index() -> HttpResponse {
    let index_obj = json_objs::OriginObj {
        code: true,
        err_message: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    HttpResponse::Ok().json(index_obj)
}

pub fn logup_cow(username: &str, email: &str, organization: &str) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Check email format
    if !email_format(email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
    }

    // Make sure not-double logup

    // Do organization authenitication

    // Pass checking, do db-storing
    println!("{}, {}, {}", username, email, organization);

    HttpResponse::Ok().json(result_obj)
}

pub fn logup_student(data: web::Json<json_objs::LogupObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Check email format
    if !email_format(&data.email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
    }

    // Make sure not-double logup

    // Do student autehnitication

    // Pass checking, do db-stroing

    HttpResponse::Ok().json(result_obj)
}

pub fn login_cow(username: &str) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Check user registered or not

    // Pass checking

    println!("{}", username);

    HttpResponse::Ok().json(result_obj)
}

pub fn login_student(username: &str) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    println!("{}", username);

    // Check user registered or not

    // Pass checking

    HttpResponse::Ok().json(result_obj)
}

// Task Manage Function Methods

pub fn release_task(_data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Check task name duplication

    // Check task mode

    // Set llimit timer

    HttpResponse::Ok().json(result_obj)
}

pub fn check_task(_data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    let result_obj = json_objs::TaskViewObj {
        code: true,
        err_message: "".to_string(),
        task_status: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn receive_task(_data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn submit_task(_data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

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

// Credit And Account Management

pub fn check_credit(_data: web::Json<json_objs::CheckCreditObj>) -> HttpResponse {
    let result_obj = json_objs::CreditScoreObj {
        code: true,
        err_message: "".to_string(),
        credit_score: 0,
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn recharge(_data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn withdraw(_data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    HttpResponse::Ok().json(result_obj)
}

// Tools Function Methods
fn email_format(email: &str) -> bool {
    let email_rg =
        Regex::new(r"^[a-zA-Z0-9_.-]+@[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*\.[a-zA-Z0-9]{2,6}$").unwrap();
    // Check email format
    match email_rg.captures(&email) {
        Some(_x) => true,
        None => false,
    }
}
