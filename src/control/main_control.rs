/*
* Emtm-Controller Modules -- Main Control
*/
extern crate chrono;
extern crate json;
extern crate regex;

use actix_web::{web, HttpResponse};
use chrono::{Local, NaiveDateTime};
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
pub fn email_format(email: &str) -> bool {
    let email_rg =
        Regex::new(r"^[a-zA-Z0-9_.-]+@[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*\.[a-zA-Z0-9]{2,6}$").unwrap();
    // Check email format
    match email_rg.captures(&email) {
        Some(_x) => true,
        None => false,
    }
}

pub fn phone_format(phone: &str) -> bool {
    let phone_rg = Regex::new(r"^1([38]\d|5[0-35-9]|7[3678])\d{8}$").unwrap();
    // Check phone format
    match phone_rg.captures(&phone) {
        Some(_x) => true,
        None => false,
    }
}

pub fn time_limit_valid(timestamp: &str) -> bool {
    let current_time = (Local::now()).naive_local();

    let task_limit_time = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d:%H-%M");

    // Check error
    let result = match task_limit_time {
        Ok(stamp) => stamp > current_time,
        Err(_) => false,
    };

    result
}

pub fn parse_str_to_naive_date_time(timestamp: &str) -> NaiveDateTime {
    let result = match NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d:%H-%M") {
        Ok(stamp) => stamp,
        Err(_) => (Local::now()).naive_local(),
    };

    result
}
