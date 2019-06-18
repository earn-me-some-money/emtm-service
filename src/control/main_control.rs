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
use emtm_db::controller::{
    user_controller::UserController, Controller, 
    school_controller_zh::SchoolControllerZh
};
use emtm_db::models::users::{User, UserId};
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

pub fn check_credit(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::CreditScoreObj {
        code: true,
        err_message: "".to_string(),
        credit_score: 0,
    };

    let db_control = Controller::new();

    let wechat_id : UserId = UserId::WechatId(&data.userid);
    let user_credit = match db_control.get_user_from_identifier(wechat_id) {
        Some(User::Cow(_)) => -1,
        Some(User::Student(stu)) => stu.credit,
        None => -1
    };

    if user_credit == -1 {
        result_obj.code = false;
        result_obj.err_message = "Cannot find target student-user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }
    else {
        result_obj.credit_score = user_credit.into();
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn recharge(data: web::Json<json_objs::RechargeObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    let wechat_id : UserId = UserId::WechatId(&data.userid);
    let target_user_exist = match db_control.get_user_from_identifier(wechat_id.clone()) {
        Some(_) => true,
        None => false
    };

    if !target_user_exist {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot reach target user with input wechat-id!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }
    // Get target user
    let mut target_user_tokens = match db_control.get_user_from_identifier(wechat_id.clone()).unwrap() {
        User::Cow(cow) => cow.tokens,
        User::Student(stu) => stu.tokens
    };

    if data.recharge_amount <= 0 {
        result_obj.code = false;
        result_obj.err_message = "Please don't make negative or zero rechargement!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }
    else {
        target_user_tokens += data.recharge_amount;
        let target_user = match db_control.get_user_from_identifier(wechat_id).unwrap() {
            User::Cow(mut cow) => {
                cow.tokens = target_user_tokens;
                User::Cow(cow)
            }
            User::Student(mut stu) => {
                stu.tokens = target_user_tokens;
                User::Student(stu)
            }
        };

        let new_user = vec![target_user];
        // Update back into db
        match &db_control.update_users(&new_user)[0] {
            Ok(_) => {}
            Err(err) => {
                result_obj.code = false;
                result_obj.err_message = format!("{}", err);
            }
        };
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn withdraw(data: web::Json<json_objs::WithdrawObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    let wechat_id : UserId = UserId::WechatId(&data.userid);
    let target_user_exist = match db_control.get_user_from_identifier(wechat_id.clone()) {
        Some(_) => true,
        None => false
    };

    if !target_user_exist {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot reach target user with input wechat-id!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }
    // Get target user
    let mut target_user_tokens = match db_control.get_user_from_identifier(wechat_id.clone()).unwrap() {
        User::Cow(cow) => cow.tokens,
        User::Student(stu) => stu.tokens
    };

    if data.withdraw_amount > target_user_tokens {
        result_obj.code = false;
        result_obj.err_message = "Error! Withdraw amount larger than user's balance!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }
    else {
        target_user_tokens -= data.withdraw_amount;
        let target_user = match db_control.get_user_from_identifier(wechat_id).unwrap() {
            User::Cow(mut cow) => {
                cow.tokens = target_user_tokens;
                User::Cow(cow)
            }
            User::Student(mut stu) => {
                stu.tokens = target_user_tokens;
                User::Student(stu)
            }
        };

        let new_user = vec![target_user];
        // Update back into db
        match &db_control.update_users(&new_user)[0] {
            Ok(_) => {}
            Err(err) => {
                result_obj.code = false;
                result_obj.err_message = format!("{}", err);
            }
        };
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn get_balance(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::BalanceObj {
        code: true,
        err_message: "".to_string(),
        balance: 0
    };

    let db_control = Controller::new();

    let user_wechat_id : UserId = UserId::WechatId(&data.userid);
    let user_balance = match db_control.get_user_from_identifier(user_wechat_id) {
        Some(User::Cow(cow)) => cow.tokens,
        Some(User::Student(stu)) => stu.tokens,
        None => -1
    };

    if user_balance < 0 {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    result_obj.balance = user_balance;

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

pub fn get_cow_info(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::CowInfoObj {
        code: true,
        err_message: "".to_string(),

        username: "".to_string(),
        email: "".to_string(),
        phone: "".to_string(),
        infos: "".to_string(),
        organization: "".to_string(),
    };

    let db_control = Controller::new();

    let user_wechat_id : UserId = UserId::WechatId(&data.userid);
    let user_exist = match db_control.get_user_from_identifier(user_wechat_id.clone()) {
        Some(User::Cow(_)) => true,
        Some(User::Student(_)) => false,
        None => false
    };

    if !user_exist {
        result_obj.code = false;
        result_obj.err_message = "Cannot find target cow user with userid!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    match db_control.get_user_from_identifier(user_wechat_id).unwrap() {
        User::Cow(cow) => {
            result_obj.username = cow.username;
            result_obj.email = cow.email;
            result_obj.phone = cow.phone;
            result_obj.infos = cow.personal_info;
            result_obj.organization = cow.company
        }
        User::Student(_) => {}
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn get_stu_info(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::StuInfoObj {
        code: true,
        err_message: "".to_string(),

        username: "".to_string(),
        email: "".to_string(),
        phone: "".to_string(),
        infos: "".to_string(),
        school_name: "".to_string(),
        student_id: "".to_string(),
        major: "".to_string(),
        year: 0,

        credit: 0,
        accepted: 0,
        finished: 0,
    };

    let db_control = Controller::new();

    let user_wechat_id : UserId = UserId::WechatId(&data.userid);
    let user_exist = match db_control.get_user_from_identifier(user_wechat_id.clone()) {
        Some(User::Cow(_)) => false,
        Some(User::Student(_)) => true,
        None => false
    };

    if !user_exist {
        result_obj.code = false;
        result_obj.err_message = "Cannot find target student user with userid!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    match db_control.get_user_from_identifier(user_wechat_id).unwrap() {
        User::Student(stu) => {
            result_obj.username = stu.username;
            result_obj.email = stu.email;
            result_obj.phone = stu.phone;
            result_obj.infos = stu.personal_info;
            result_obj.school_name = db_control.get_school_name(stu.school_id).unwrap();
            result_obj.student_id = stu.student_id;
            result_obj.major = stu.major;
            result_obj.year = stu.year;

            result_obj.credit = stu.credit;
            result_obj.accepted = stu.accepted;
            result_obj.finished = stu.finished;
        }
        User::Cow(_) => {}
    };

    HttpResponse::Ok().json(result_obj)
}

pub fn edit_cow_info(data: web::Json<json_objs::CowEditInfoObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string()
    };

    let db_control = Controller::new();

    let user_wechat_id : UserId = UserId::WechatId(&data.userid);
    let cow_exist = match db_control.get_user_from_identifier(user_wechat_id.clone()) {
        Some(User::Cow(_)) => true,
        Some(User::Student(_)) => false,
        None => false
    };

    if !cow_exist {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target cow-user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check new_email dupliation
    let check_user_email: UserId = UserId::Email(&data.new_email);
    let email_dup = match db_control.get_user_from_identifier(check_user_email) {
        Some(x) => match x {
            User::Cow(cow) => cow.wechat_id != data.userid,
            User::Student(stu) => stu.wechat_id != data.userid
        },
        None => false
    };

    if email_dup {
        result_obj.code = false;
        result_obj.err_message = "Error! New Email has duplication with other users!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check new phone duplication
    let check_user_phone: UserId = UserId::Phone(&data.new_phone);
    let phone_dup = match db_control.get_user_from_identifier(check_user_phone) {
        Some(x) => match x {
            User::Cow(cow) => cow.wechat_id != data.userid,
            User::Student(stu) => stu.wechat_id != data.userid
        },
        None => false
    };

    if phone_dup {
        result_obj.code = false;
        result_obj.err_message = "Error! New Phone has duplication with other users!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    match db_control.get_user_from_identifier(user_wechat_id).unwrap() {
        User::Cow(mut cow) => {
            cow.email = data.new_email.clone();
            cow.phone = data.new_phone.clone();
            cow.personal_info = data.new_infos.clone();
            let update_users = vec![User::Cow(cow)];
            match &db_control.update_users(&update_users)[0] {
                Ok(_) => {}
                Err(err) => {
                    result_obj.code = false;
                    result_obj.err_message = format!("{}", err);
                }
            }
        }
        User::Student(_) => {}
    }


    HttpResponse::Ok().json(result_obj)
}

pub fn edit_stu_info(data: web::Json<json_objs::StuEditInfoObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string()
    };

    let db_control = Controller::new();

    let user_wechat_id : UserId = UserId::WechatId(&data.userid);
    let cow_exist = match db_control.get_user_from_identifier(user_wechat_id.clone()) {
        Some(User::Cow(_)) => false,
        Some(User::Student(_)) => true,
        None => false
    };

    if !cow_exist {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target student-user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check new_email dupliation
    let check_user_email: UserId = UserId::Email(&data.new_email);
    let email_dup = match db_control.get_user_from_identifier(check_user_email) {
        Some(x) => match x {
            User::Cow(cow) => cow.wechat_id != data.userid,
            User::Student(stu) => stu.wechat_id != data.userid
        },
        None => false
    };

    if email_dup {
        result_obj.code = false;
        result_obj.err_message = "Error! New Email has duplication with other users!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check new phone duplication
    let check_user_phone: UserId = UserId::Phone(&data.new_phone);
    let phone_dup = match db_control.get_user_from_identifier(check_user_phone) {
        Some(x) => match x {
            User::Cow(cow) => cow.wechat_id != data.userid,
            User::Student(stu) => stu.wechat_id != data.userid
        },
        None => false
    };

    if phone_dup {
        result_obj.code = false;
        result_obj.err_message = "Error! New Phone has duplication with other users!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    

    match db_control.get_user_from_identifier(user_wechat_id).unwrap() {
        User::Student(mut stu) => {
            stu.email = data.new_email.clone();
            stu.phone = data.new_phone.clone();
            stu.personal_info = data.new_infos.clone();
            stu.major = data.new_major.clone();
            stu.year = data.new_year;
            let update_users = vec![User::Student(stu)];
            match &db_control.update_users(&update_users)[0] {
                Ok(_) => {}
                Err(err) => {
                    result_obj.code = false;
                    result_obj.err_message = format!("{}", err);
                }
            }
        }
        User::Cow(_) => {}
    }


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
