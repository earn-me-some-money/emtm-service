/*
* Emtm-Controller Modules -- Main Control
*/
extern crate chrono;
extern crate json;
extern crate regex;

use std::cell::Cell;
use std::time::Duration;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::client::PayloadError;
use actix_web::{
    client::{Client, SendRequestError},
    http::StatusCode,
    web, HttpResponse,
};
use chrono::{Local, NaiveDateTime};
use emtm_verify::Verifier;
use futures::{future::lazy, Future, future::result};
use log::*;
use regex::Regex;
use serde::*;

use crate::control::json_objs;

static TX_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[derive(Debug)]
pub enum APIError {
    RequestError(SendRequestError),
    ServerResponseError(PayloadError),
    ServerStatusError(String),
}

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

pub fn verify(_data: Multipart, counter: web::Data<Cell<usize>>) -> HttpResponse {
    let result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // data
    //     .map_err(error::ErrorInternalServerError)
    //     .map(|field| save_file(field).into_stream())
    //     .flatten()
    //     .collect()
    //     .map(|sizes| HttpResponse::Ok().json(sizes))
    //     .map_err(|e| {
    //         println!("failed: {}", e);
    //         e
    //     })

    /*
    let verifier = Verifier::new();

    // Process with multipart datas
    data.map_err(error::ErrorInternalServerError).map(|field| {

    });

    let verify_res = verifier.verify(&data.image_data, &data.user_id, Some(&data.organization));
    let verify_result = match verify_res {
        Ok(_) => "",
        Err(err) => err
    };

    if verify_result.len() > 0 {
        result_obj.code = false;
        result_obj.err_message = verify_result;
    }
    */

    HttpResponse::Ok().json(result_obj)
}

// Get User's wechat openid
pub fn get_wechatid(
    data: web::Json<json_objs::GetWechatIdObj>,
) -> Box<Future<Item = HttpResponse, Error=actix_web::Error>> {
    let mut result_obj = json_objs::WechatIdResultObj {
        openid: "".to_string(),
        errcode: 0,
        errmsg: "".to_string(),
    };

    let empty_form = json_objs::ResponseForm {
        openid: "Error".to_string(),
        errcode: 0,
        errmsg: "Error".to_string(),
        session_key: "Error".to_string(),
        unionid: "Error".to_string(),
    };

    let params = json_objs::RequestForm {
        appid: data.appid.clone(),
        secret: data.secret.clone(),
        js_code: data.code.clone(),
        grant_type: "authorization_code".to_string(),
    };

    let ret = api_request(&params)
        .then(|response| {
            if let Err(err) = response {
                return Ok(HttpResponse::BadGateway().json(json_objs::WechatIdResultObj {
                    openid: "".to_string(),
                    errcode: -1,
                    errmsg: format!("{:?}", err),
                }));
            }
            let response = response.unwrap();
            let mut api_response_correct = true;
            let api_result: json_objs::ResponseForm = match serde_json::from_str(&response) {
                Ok(r) => r,
                Err(e) => {
                    debug!("Failed to parse json: {}", e);
                    api_response_correct = false;
                    empty_form
                }
            };

            if api_response_correct {
                result_obj.errcode = api_result.errcode;
                result_obj.openid = api_result.openid;
                result_obj.errmsg = api_result.errmsg;
            }

            Ok(HttpResponse::Ok().json(result_obj))
        });
    Box::new(ret)
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

pub fn api_request(
    params: &json_objs::RequestForm,
) -> Box<Future<Item = String, Error = APIError>> {
    let mut client_builder = Client::build();
    client_builder = client_builder.timeout(Duration::from_secs(20));

    let client = client_builder.finish();

    let ret = client
        .get(TX_URL)
        .send_form(params)
        .map_err(|error| {
            warn!("Error {:?} when requesting tx getting wechatid api!", error);
            APIError::RequestError(error)
        })
        .and_then(|mut response| {
            debug!("Response header: {:?}", response);
            match response.status() {
                StatusCode::OK => match response.body().wait() {
                    Ok(item) => Ok(String::from_utf8_lossy(&item[..]).into_owned()),
                    Err(err) => Err(APIError::ServerResponseError(err)),
                },
                _ => Err(APIError::ServerStatusError(format!(
                    "Tencent Server Response code: {}",
                    response.status().as_u16()
                ))),
            }
        });
    Box::new(ret)
}
