/*
* Emtm-Controller Modules
*/
extern crate chrono;
extern crate emtm_db;
extern crate json;
extern crate regex;

use actix_web::{web, HttpResponse};
use chrono::{Local, NaiveDateTime};
use regex::Regex;

use crate::control::json_objs;
use emtm_db::controller::{
    mission_controller::MissionController, school_controller_zh::SchoolControllerZh,
    user_controller::UserController, Controller,
};
// Model Schemas
use emtm_db::models::missions::{Mission, MissionType, PartState, Participant};
use emtm_db::models::users::{Cow, Student, User, UserId};
use emtm_db::search;

const SUPPORT_TASK_KINDS: i8 = 3;

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
    let login_user_id: UserId = UserId::WechatId(userid);
    let login_enable = match db_control.get_user_from_identifier(login_user_id) {
        Some(_x) => match _x {
            User::Cow(_cow) => !mode,
            User::Student(_stu) => mode,
        },
        None => false,
    };

    // Pass checking
    if !login_enable {
        result_obj.code = false;
        result_obj.err_message = "Login Fail! Taret User Not Registered!".to_string();
    }

    HttpResponse::Ok().json(result_obj)
}

// Task Manage Function Methods

pub fn release_task(data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Init DB Control
    let db_control = Controller::new();

    // Get current user's database-user-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let database_user_id = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(cow)) => cow.uid,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    // Error Checking -- User Existence
    if database_user_id == -1 {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Define Task_Release error message
    let error_types = [
        "Task Name Duplication",
        "Task Mode Invalid",
        "Task Pay Can not be Negative",
        "Task Time-Limit Invalid",
        "Task Max-Participants Number Should be Positive",
    ];

    let mut error_index = 5;
    let exist_posted_tasks = db_control.get_poster_missions(database_user_id);
    // Check task name duplication
    for task in exist_posted_tasks.iter() {
        if task.name == data.task_name {
            error_index = 0;
        }
    }

    // Check task mode valid
    if data.task_mode >= SUPPORT_TASK_KINDS || data.task_mode < 0 {
        error_index = 1;
    }

    // Check payment positive
    if data.task_pay <= 0 {
        error_index = 2;
    }

    // Check timelimit valid -- cannot before current time
    if !time_limit_valid(&data.task_time_limit) {
        error_index = 3;
    }

    if data.task_request.max_participants <= 0 {
        error_index = 4;
    }

    if error_index < 5 {
        result_obj.code = false;
        result_obj.err_message = ["Error!", error_types[error_index]].join(" ").to_string();
        return HttpResponse::Ok().json(result_obj);
    } else {
        // According to release mode, read task_request

        // Pass all checking, store into db
        let mission = Mission {
            mid: 0,
            poster_uid: database_user_id,
            bounty: data.task_pay,
            risk: data.task_risk,
            name: data.task_name.clone(),
            mission_type: MissionType::from_val(data.task_mode),
            content: data.task_intro.clone(),
            post_time: (Local::now()).naive_local(),
            deadline: parse_str_to_naive_date_time(&data.task_time_limit),
            participants: vec![],
            max_participants: data.task_request.max_participants,
        };

        if let Err(err) = db_control.add_mission(&mission) {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    }

    if result_obj.code {
        // Set limit timer

    }

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

pub fn receive_task(data: web::Json<json_objs::ReceiveTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Init DB Control
    let db_control = Controller::new();

    // Find student database-id by wechat-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let database_user_id = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(_cow)) => -1,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    if database_user_id == -1 {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target student in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check mission participant duplication
    let wechat_releaser_id: UserId = UserId::WechatId(&data.target_userid);
    let target_releaser_id = match db_control.get_user_from_identifier(wechat_releaser_id) {
        Some(User::Cow(cow)) => cow.uid,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    let missions_collection = db_control.get_poster_missions(target_releaser_id);

    let mut task_mid = -1;
    for task in missions_collection.iter() {
        if task.name == data.target_task {
            task_mid = task.mid;
        }
    }

    if task_mid != -1 {
        // Check duplication
        let participants = db_control.get_mission_participants(task_mid);
        for person in participants.iter() {
            if person.student_uid == database_user_id {
                result_obj.code = false;
                result_obj.err_message = "Error! Task Participant Duplication!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }
        // Check student condition satisify

        // Check task exceed max_participants or not
        let mut find_mission = false;
        match db_control.get_mission_from_mid(task_mid) {
            Some(x) => {
                find_mission = true;
                result_obj.code = x.max_participants > (participants.len() as i32);
            }
            None => {
                result_obj.code = false;
                result_obj.err_message = "Error! Target Mission Not Exist!".to_string();
            }
        };

        if find_mission && !result_obj.code {
            result_obj.err_message = "Error! Target Task Exceed Max Participants Size!".to_string();
        }

        // Pass Checking, store participant into db
        if result_obj.code {
            let new_part_user = vec![Participant {
                student_uid: database_user_id,
                state: PartState::from_val(0),
            }];

            if let Err(err) = db_control.add_participants(task_mid, &new_part_user) {
                result_obj.code = false;
                result_obj.err_message = format!("{}", err);
            }
        }
    } else {
        result_obj.code = false;
        result_obj.err_message =
            "Error! Cannot match the mission name with target releaser!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn search_mission(data: web::Json<json_objs::MissionSearchObj>) -> HttpResponse {
    let mut result_obj = json_objs::SearchResultObj {
        code: true,
        err_message: "".to_string(),
        search_result: vec![],
    };

    let db_control = Controller::new();
    // Search with database-searcher
    let search_result = search::query_mission(&data.keyword);

    let result_vector = match search_result {
        Ok(result) => result,
        Err(_) => vec![],
    };

    // Check search error or not
    if result_vector.len() == 0 {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot match any mission with search keyword!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Parse result vector
    for ele in result_vector.iter() {
        // Search mission with element's mid
        match db_control.get_mission_from_mid(ele.0) {
            Some(the_mission) => {
                // Find poster wechat id
                let poster_id: UserId = UserId::Uid(the_mission.poster_uid);
                let poster_wechatid = match db_control.get_user_from_identifier(poster_id) {
                    Some(User::Cow(cow)) => cow.wechat_id,
                    Some(User::Student(stu)) => stu.wechat_id,
                    None => "".to_string(),
                };

                // Check wechat-id successfully get
                if poster_wechatid.len() == 0 {
                    result_obj.code = false;
                    result_obj.err_message =
                        "Error! Cannot get target mission-poster's wechat id!".to_string();
                    return HttpResponse::Ok().json(result_obj);
                }

                // Push new search result into response
                let new_search_result = json_objs::SearchElementObj {
                    mid: ele.0,
                    name: the_mission.name,
                    content: the_mission.content,
                    poster_userid: poster_wechatid,
                    time_limit: the_mission.deadline.to_string(),
                    score: ele.1,
                };
                result_obj.search_result.push(new_search_result);
            }
            None => (),
        };
    }

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

fn time_limit_valid(timestamp: &str) -> bool {
    let current_time = (Local::now()).naive_local();

    let task_limit_time = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d:%H-%M");

    // Check error
    let result = match task_limit_time {
        Ok(stamp) => stamp > current_time,
        Err(_) => false,
    };

    result
}

fn parse_str_to_naive_date_time(timestamp: &str) -> NaiveDateTime {
    let result = match NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d:%H-%M") {
        Ok(stamp) => stamp,
        Err(_) => (Local::now()).naive_local(),
    };

    result
}
