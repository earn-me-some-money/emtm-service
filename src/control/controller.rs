/*
* Emtm-Controller Modules
*/
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
    let index_obj = json_objs::OriginObj {
        code: true,
        err_message: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    // Check email format
    if email_format(email) {
        println!("Email Pass Checking~")
    }

    println!("{}, {}, {}", username, email, organization);

    // Make sure not-double logup

    // Do organization authenitication

    // Pass checking, do db-stroing

    HttpResponse::Ok().json(index_obj)
}

pub fn logup_student(data: web::Json<json_objs::LogupObj>) -> HttpResponse {
    // Check email format
    if email_format(&data.email) {
        println!("Email Pass Checking~")
    }

    // Make sure not-double logup

    // Do student autehnitication

    // Pass checking, do db-stroing

    HttpResponse::Ok().json(data.0)
}

pub fn login_cow(username: &str) -> HttpResponse {
    let index_obj = json_objs::OriginObj {
        code: true,
        err_message: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    println!("{}", username);

    // Check user registered or not

    // Pass checking

    HttpResponse::Ok().json(index_obj)
}

pub fn login_student(username: &str) -> HttpResponse {
    let index_obj = json_objs::OriginObj {
        code: true,
        err_message: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    println!("{}", username);

    // Check user registered or not

    // Pass checking

    HttpResponse::Ok().json(index_obj)
}

// Task Manage Function Methods

pub fn release_task(data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn check_task(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn receive_task(data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn submit_task(data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

// Student Friend && Group Manage Function Methods

pub fn create_group(data: web::Json<json_objs::CreateGroupObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn join_group(data: web::Json<json_objs::JoinGroupObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn add_friend(data: web::Json<json_objs::AddFriendObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

// Credit And Account Management

pub fn check_credit(data: web::Json<json_objs::CheckCreditObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn recharge(data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
}

pub fn withdraw(data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    HttpResponse::Ok().json(data.0)
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
