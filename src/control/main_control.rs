/*
* Emtm-Controller Modules -- Main Control
*/

use std::time::Duration;


use actix_web::client::PayloadError;
use actix_web::{
    client::{Client, SendRequestError},
    http::StatusCode,
    web, HttpResponse,
};
use chrono::{Local, NaiveDateTime};
use emtm_verify::Verifier;
use futures::{future, Future};
use log::*;
use regex::Regex;


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

pub fn verify(
    data: web::Json<json_objs::VerifyInfo>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let mut result_obj = json_objs::OriginObj {
        code: false,
        err_message: "".to_string(),
    };

    let raw_data = match base64::decode(data.image_data.as_bytes()) {
        Ok(raw) => raw,
        Err(err) => {
            result_obj.err_message = format!("Failed to decode image data: {:?}", err);
            return Box::new(future::ok(HttpResponse::BadRequest().json(result_obj)));
        }
    };

    let verifier = Verifier::new();

    let _id = match data.verify_mode {
        true => Some(&data.user_id),
        false => None,
    };

    Box::new(
        verifier
            .verify(&raw_data, &data.organization, Some(&data.organization))
            .then(|verify_result| {
                if let Err(err) = verify_result {
                    result_obj.err_message = format!("Verification failed: {:?}", err);
                    future::ok(HttpResponse::BadRequest().json(result_obj))
                } else {
                    result_obj.code = true;
                    future::ok(HttpResponse::Ok().json(result_obj))
                }
            }),
    )
}

// Get User's wechat openid
pub fn get_wechatid(
    data: web::Json<json_objs::GetWechatIdObj>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let mut result_obj = json_objs::WechatIdResultObj {
        openid: "None".to_string(),
        errcode: 0,
        errmsg: "None".to_string(),
    };

    let empty_form = json_objs::ResponseForm {
        openid: None,
        errcode: None,
        errmsg: None,
        unionid: None,
        session_key: None,
    };

    let params: [&str; 4] = [&data.appid, &data.secret, &data.code, "authorization_code"];

    let ret = api_request(&params).then(|response| {
        if let Err(err) = response {
            return Ok(
                HttpResponse::BadGateway().json(json_objs::WechatIdResultObj {
                    openid: "".to_string(),
                    errcode: -1,
                    errmsg: format!("{:?}", err),
                }),
            );
        }
        let response = response.unwrap();
        let mut api_response_correct = true;
        let api_result: json_objs::ResponseForm = match serde_json::from_str(&response) {
            Ok(r) => r,
            Err(_e) => {
                api_response_correct = false;
                empty_form
            }
        };

        if api_response_correct {
            result_obj.errcode = api_result.errcode.unwrap_or(0);
            result_obj.openid = api_result.openid.unwrap_or("None".to_string());
            result_obj.errmsg = api_result.errmsg.unwrap_or("None".to_string());
        } else {
            result_obj.errcode = api_result.errcode.unwrap_or(0);
            result_obj.errmsg = api_result.errmsg.unwrap_or("None".to_string());
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

pub fn api_request(params: &[&str]) -> Box<Future<Item = String, Error = APIError>> {
    let mut client_builder = Client::build();
    client_builder = client_builder.timeout(Duration::from_secs(20));

    let client = client_builder.finish();

    let names = vec!["appid", "secret", "js_code", "grant_type"];
    let url_encode = names
        .iter()
        .zip(params)
        .map(|(name, param)| [*name, *param].join("="))
        .collect::<Vec<String>>()
        .join("&");
    let target_url = [TX_URL, &url_encode].join("?");

    let ret = client
        .get(target_url)
        .send()
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
