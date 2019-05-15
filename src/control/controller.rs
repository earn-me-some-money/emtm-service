/*
* Emtm-Controller Modules
*/
extern crate emtm_db;
extern crate json;
extern crate regex;

use actix_web::{web, HttpResponse};
use regex::Regex;

use crate::control::json_objs;
use emtm_db::controller::{
    school_controller_zh::SchoolControllerZh, user_controller::UserController, Controller,
};
use emtm_db::models::users::{Cow, Student, UserId, User};

// Basic Function Methods

pub fn index() -> HttpResponse {
    let index_obj = json_objs::OriginObj {
        code: true,
        err_message: "Welcome to Emtm-Server Index Page~".to_string(),
    };

    HttpResponse::Ok().json(index_obj)
}

pub fn logup_cow(
    username: &str,
    userid: &str,
    phone: &str,
    email: &str,
    infos: &str,
    organization: &str,
) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };
    // Init DB Control
    let db_control = Controller::new();
    // Define Duplication error message
    let dup_errors = ["UserID", "Email", "Phone", "Logup Error!", "Duplication!"];
    // Variable order: userid, email, organization
    let mut dup_array = [false, false, false];
    let mut logup_enable = true;

    // Check email format
    if !email_format(email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else if !phone_format(phone) {
        // Checking phone format
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Phone Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check registered infos duplication
    let check_user_id: UserId = UserId::WechatId(userid);
    match db_control.get_user_from_identifier(check_user_id) {
        Some(_x) => dup_array[0] = true,
        None => (),
    }

    let check_user_email: UserId = UserId::Email(email);
    match db_control.get_user_from_identifier(check_user_email) {
        Some(_x) => dup_array[1] = true,
        None => (),
    }

    let check_user_phone: UserId = UserId::Phone(phone);
    match db_control.get_user_from_identifier(check_user_phone) {
        Some(_x) => dup_array[2] = true,
        None => (),
    }

    for index in 0..3 {
        if dup_array[index] {
            logup_enable = false;
            result_obj.err_message = [dup_errors[3], dup_errors[index], dup_errors[4]]
                .join(" ")
                .to_string();
            break;
        }
    }

    // Do organization authenitication

    // Pass checking, do db-storing
    if logup_enable {
        // New Adding Cow Vector
        let cows = vec![Cow {
            uid: 0,
            wechat_id: userid.to_string(),
            phone: phone.to_string(),
            personal_info: infos.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            verified: false,
            tokens: 0,
            company: organization.to_string(),
        }];

        if let Err(err) = db_control.add_cows(&cows).remove(0) {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn logup_student(data: web::Json<json_objs::StuLogupObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };
    // Init DB Control
    let db_control = Controller::new();
    // Define Duplication error message
    let dup_errors = [
        "UserID",
        "Email",
        "Phone",
        "School_Info",
        "Logup Error!",
        "Duplication!",
    ];
    // Variable order: userid, email, organization
    let mut dup_array = [false, false, false, false];
    let mut logup_enable = true;

    // Check email format
    if !email_format(&data.email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else if !phone_format(&data.phone) {
        // Checking phone format
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Phone Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Make sure not-double logup
    // Check registered infos duplication
    let check_user_id: UserId = UserId::WechatId(&data.userid);
    match db_control.get_user_from_identifier(check_user_id) {
        Some(_x) => dup_array[0] = true,
        None => (),
    }

    let check_user_email: UserId = UserId::Email(&data.email);
    match db_control.get_user_from_identifier(check_user_email) {
        Some(_x) => dup_array[1] = true,
        None => (),
    }

    let check_user_phone: UserId = UserId::Phone(&data.phone);
    match db_control.get_user_from_identifier(check_user_phone) {
        Some(_x) => dup_array[2] = true,
        None => (),
    }

    // Search school id by school_name
    let school_id = match db_control.get_school_id(&data.school_name) {
        Some(_x) => _x,
        None => -1,
    };

    if school_id == -1 {
        result_obj.code = false;
        result_obj.err_message = "School Name Error! Cannot search target school id...".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    let check_user_school_info: UserId = UserId::SchoolInfo(school_id, &data.student_id);
    match db_control.get_user_from_identifier(check_user_school_info) {
        Some(_x) => dup_array[3] = true,
        None => (),
    }

    for index in 0..4 {
        if dup_array[index] {
            logup_enable = false;
            result_obj.err_message = [dup_errors[4], dup_errors[index], dup_errors[5]]
                .join(" ")
                .to_string();
            break;
        }
    }

    // Do student autehnitication

    // Pass checking, do db-stroing
    if logup_enable {
        // New Adding Cow Vector
        let students = vec![Student {
            uid: 0,
            wechat_id: data.userid.clone(),
            phone: data.phone.clone(),
            personal_info: data.infos.clone(),
            email: data.email.clone(),
            username: data.username.clone(),
            verified: false,
            tokens: 0,
            school_id: school_id.clone(),
            student_id: data.student_id.clone(),
            credit: 100, // Init Credit Score is 100%
            accepted: 0,
            finished: 0,
            major: data.major.clone(),
            year: data.year.clone(),
        }];

        if let Err(err) = db_control.add_students(&students).remove(0) {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn login(userid: &str, mode: bool) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Init DB Control
    let db_control = Controller::new();

    // Check user registered or not
    let login_user_id : UserId = UserId::WechatId(userid);
    let login_enable = match db_control.get_user_from_identifier(login_user_id) {
        Some(_x) => {
            match _x {
                User::Cow(_cow) => !mode,
                User::Student(_stu) => mode,
            }
        }
        None => false
    };

    // Pass checking
    if !login_enable {
        result_obj.code = false;
        result_obj.err_message = "Login Fail! Taret User Not Registered!".to_string();
    }

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

fn phone_format(phone: &str) -> bool {
    let phone_rg = Regex::new(r"^1([38]\d|5[0-35-9]|7[3678])\d{8}$").unwrap();
    // Check phone format
    match phone_rg.captures(&phone) {
        Some(_x) => true,
        None => false,
    }
}
