/*
* Emtm-Controller Modules -- Task Control
*/
extern crate chrono;
extern crate emtm_db;
extern crate json;

use actix_web::{web, HttpResponse};
use chrono::Local;

use crate::control::json_objs;
use crate::control::main_control;
use emtm_db::controller::{
    errand_controller::ErrandController, mission_controller::MissionController,
    school_controller_zh::SchoolControllerZh, survey_controller::SurveyController,
    trade_controller::TradeController, user_controller::UserController, Controller,
};
// Model Schemas
use emtm_db::models::errand::Errand;
use emtm_db::models::missions::{Mission, MissionType, PartState, Participant};
use emtm_db::models::survey::{Answer, AnswerContent, Question, QuestionContent};
use emtm_db::models::trade::Trade;
use emtm_db::models::users::{User, UserId};

use emtm_db::search;

const SUPPORT_TASK_KINDS: i8 = 3;

// Task Manage Function Methods

pub fn release_task(data: web::Json<json_objs::ReleaseTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::MissionOkObj {
        code: true,
        err_message: "".to_string(),
        mid: 0,
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
    if data.task_pay <= 0 && data.task_mode != 1 {
        error_index = 2;
    }

    // Check timelimit valid -- cannot before current time
    if !main_control::time_limit_valid(&data.task_time_limit) {
        error_index = 3;
    }

    match data.task_request.max_participants {
        Some(max_parts) if max_parts <= 0 => {
            error_index = 4;
        }
        _ => (),
    };

    if error_index < 5 {
        result_obj.code = false;
        result_obj.err_message = ["Error!", error_types[error_index]].join(" ").to_string();
        return HttpResponse::Ok().json(result_obj);
    } else {
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
            deadline: main_control::parse_str_to_naive_date_time(&data.task_time_limit),
            participants: vec![],
            max_participants: data.task_request.max_participants,
            min_grade: data.task_request.min_grade,
            max_grade: data.task_request.max_grade,
            school: match &data.task_request.school {
                Some(x) => db_control.get_school_id(&x),
                None => None,
            },
            min_finished: data.task_request.task_expe,
            major: data.task_request.major.clone(),
            min_credit: data.task_request.credit_score,
        };

        match db_control.add_mission(&mission) {
            Ok(mid) => {
                result_obj.mid = mid;
            }
            Err(err) => {
                result_obj.code = false;
                result_obj.err_message = format!("{}", err);
            }
        };
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn release_task_question(data: web::Json<json_objs::QuestionNaireObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    // Check mission_id validation
    match db_control.get_mission_from_mid(data.mid) {
        Some(task) => {
            // Check mission releaser is student or not
            let poster_uid = task.poster_uid;
            let is_stu = match db_control.get_user_from_identifier(UserId::Uid(poster_uid)) {
                Some(User::Cow(_)) => true,
                Some(User::Student(_)) => false,
                None => false,
            };
            if !is_stu {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Target mission poster is not a cow user!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find mission with target mid!".to_string();
            return HttpResponse::Ok().json(result_obj);
        }
    }

    // Make questions list
    let mut questions = vec![];
    for question in data.questions.iter() {
        // Make Question choices content
        let db_question_content = match &question.choices {
            Some(x) => {
                if question.q_type == 1 {
                    QuestionContent::SingleChoice(x.to_vec())
                } else {
                    QuestionContent::MultiChoice(x.to_vec())
                }
            }
            None => QuestionContent::Blank,
        };

        let db_question = Question {
            mid: data.mid,
            description: question.content.clone(),
            choices: db_question_content,
        };
        questions.push(db_question);
    }

    // Check current release mission has no other records
    let has_record = match db_control.get_errand(data.mid) {
        Some(_) => true,
        None => match db_control.get_trade(data.mid) {
            Some(_) => true,
            None => false,
        },
    };
    if has_record {
        result_obj.code = false;
        result_obj.err_message =
            "Duplicatioin Error! Database already has target mission's record!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Push into db
    match db_control.add_questions(questions, data.mid) {
        Ok(_) => {}
        Err(_) => {
            result_obj.code = false;
            result_obj.err_message =
                "Duplicatioin Error! Cannot rewrite questionnaire task detail!".to_string();
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn release_task_transaction(data: web::Json<json_objs::TransactionObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    // Check mission_id validation
    match db_control.get_mission_from_mid(data.mid) {
        Some(task) => {
            // Check mission releaser is student or not
            let poster_uid = task.poster_uid;
            let is_stu = match db_control.get_user_from_identifier(UserId::Uid(poster_uid)) {
                Some(User::Cow(_)) => false,
                Some(User::Student(_)) => true,
                None => false,
            };
            if !is_stu {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Target mission poster is not a student user!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find mission with target mid!".to_string();
            return HttpResponse::Ok().json(result_obj);
        }
    }

    // Make trade objects
    let trade = Trade {
        mid: data.mid,
        item_type: data.t_type.clone(),
        item_info: data.info.clone(),
        item_condition: data.loss,
        address: data.address.clone(),
    };

    // Check current release mission has no other records
    let has_record = match db_control.get_questionnaire(data.mid) {
        Ok(questions) => {
            if questions.len() > 0 {
                true
            } else {
                match db_control.get_errand(data.mid) {
                    Some(_) => true,
                    None => false,
                }
            }
        }
        Err(_) => true,
    };
    if has_record {
        result_obj.code = false;
        result_obj.err_message =
            "Duplicatioin Error! Database already has target mission's record!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    match db_control.add_trade(&trade) {
        Ok(_) => {}
        Err(_) => {
            result_obj.code = false;
            result_obj.err_message =
                "Duplicatioin Error! Cannot rewrite transaction task detail!".to_string();
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn release_task_errand(data: web::Json<json_objs::ErrandObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    // Check mission_id validation
    match db_control.get_mission_from_mid(data.mid) {
        Some(task) => {
            // Check mission releaser is student or not
            let poster_uid = task.poster_uid;
            let is_stu = match db_control.get_user_from_identifier(UserId::Uid(poster_uid)) {
                Some(User::Cow(_)) => false,
                Some(User::Student(_)) => true,
                None => false,
            };
            if !is_stu {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Target mission poster is not a student user!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find mission with target mid!".to_string();
            return HttpResponse::Ok().json(result_obj);
        }
    }

    // Make trade objects
    let errand = Errand {
        mid: data.mid,
        pickup_address: data.pickup_address.clone(),
        phone_number: data.phone_number.clone(),
        item_code: Some(data.pick_number.clone()),
        deliver_address: data.deliver_address.clone(),
        other_info: data.info.clone(),
    };

    // Check current release mission has no other records
    let has_record = match db_control.get_questionnaire(data.mid) {
        Ok(questions) => {
            if questions.len() > 0 {
                true
            } else {
                match db_control.get_trade(data.mid) {
                    Some(_) => true,
                    None => false,
                }
            }
        }
        Err(_) => true,
    };
    if has_record {
        result_obj.code = false;
        result_obj.err_message =
            "Duplicatioin Error! Database already has target mission's record!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    match db_control.add_errand(&errand) {
        Ok(_) => {}
        Err(_) => {
            result_obj.code = false;
            result_obj.err_message =
                "Duplicatioin Error! Cannot rewrite errand task detail!".to_string();
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn check_task(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::TaskDetailObj {
        code: false,
        err_message: "".to_string(),
        // Brief description
        mid: None,
        poster_id: None,
        poster_name: None,
        task_state: None,
        task_user_state: None,
        task_name: None,
        task_intro: None,
        task_mode: None,
        task_pay: None,
        task_time_limit: None,
        // More infos
        task_risk: None,
        task_request: None,
        // Accepter and Finisher list
        accept_users: None,
        finish_users: None,
    };

    // Init db-control
    let db_control = Controller::new();

    // Get target user's database-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let database_user_id = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(cow)) => cow.uid,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    // Handle error
    if database_user_id == -1 {
        result_obj.err_message = "Error! Can not find current user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    let wechat_poster_id: UserId = UserId::Uid(data.poster_id);
    let database_poster_id = match db_control.get_user_from_identifier(wechat_poster_id) {
        Some(User::Cow(cow)) => cow.uid,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    let mut poster_wechatid = "".to_string();
    // Handle error
    if database_poster_id == -1 {
        result_obj.err_message = "Error! Can not find mission poster in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else {
        let wechat_poster_id_1: UserId = UserId::Uid(data.poster_id);
        let database_poster_name = match db_control.get_user_from_identifier(wechat_poster_id_1) {
            Some(User::Cow(cow)) => {
                poster_wechatid = cow.wechat_id;
                cow.username
            }
            Some(User::Student(stu)) => {
                poster_wechatid = stu.wechat_id;
                stu.username
            }
            None => "None".to_string(),
        };
        result_obj.poster_name = Some(database_poster_name);
    }

    // Get target mission's mid
    let missions_collection = db_control.get_poster_missions(database_poster_id);

    let mut have_the_mission = false;
    for task in missions_collection.iter() {
        if task.mid == data.task_mid {
            have_the_mission = true;
            // Set mission parameters
            result_obj.mid = Some(task.mid);
            result_obj.poster_id = Some(data.poster_id.clone());

            result_obj.task_request = Some(json_objs::TaskRequestObj {
                min_grade: task.min_grade,
                max_grade: task.max_grade,
                major: task.major.clone(),
                school: db_control.get_school_name(task.school.unwrap_or(0)),
                task_expe: task.min_finished,
                credit_score: task.min_credit,
                max_participants: task.max_participants,
            });
            result_obj.task_name = Some(task.name.clone());
            result_obj.task_intro = Some(task.content.clone());
            result_obj.task_mode = Some(task.mission_type.to_val().into());
            result_obj.task_risk = Some(task.risk);
            result_obj.task_pay = Some(task.bounty);
            result_obj.task_time_limit = Some(task.deadline.to_string());
        }
    }

    // Handle error
    if !have_the_mission {
        result_obj.err_message =
            "Error! Target poster haven't release mission with target mid!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check target mission time state
    let mut database_mission_error = false;
    let mut over_time = false;
    match db_control.get_mission_from_mid(data.task_mid) {
        Some(mission) => over_time = mission.deadline < (Local::now()).naive_local(),
        None => {
            database_mission_error = true;
        }
    };

    // Handle error
    if database_mission_error {
        result_obj.err_message = "DataBase Error! Can not reach target mission infos!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else {
        if over_time {
            result_obj.task_state = Some(false);
        } else {
            result_obj.task_state = Some(true);
        }
    }

    // Define task_user_state
    let mut task_user_state = 3;
    if poster_wechatid == data.userid {
        // Set user state to poster
        task_user_state = 0;
    }

    // Find participant's finish state
    let participants = db_control.get_mission_participants(data.task_mid);
    let mut accept_users = json_objs::TaskAccepterObj {
        accept_user_num: 0,
        accept_user_names: vec![],
        accept_user_id: vec![],
    };
    let mut finish_users = json_objs::TaskFinisherObj {
        finish_user_num: 0,
        finish_user_names: vec![],
        finish_user_id: vec![],
    };

    for person in participants.iter() {
        // Find person's wechat-id by their database-id
        let database_person_id: UserId = UserId::Uid(person.student_uid);
        let participant_state = person.state;

        let (person_id, person_name) = match db_control.get_user_from_identifier(database_person_id)
        {
            Some(User::Student(stu)) => (stu.wechat_id, stu.username),
            Some(User::Cow(_)) => ("".to_string(), "".to_string()),
            None => ("".to_string(), "".to_string()),
        };

        // Handle Error
        if person_id == "".to_string() {
            result_obj.err_message =
                "DataBase Error! Can not reach mission's participants infos!".to_string();
            return HttpResponse::Ok().json(result_obj);
        } else {
            if person_id == data.userid {
                // Set user state to unfinished participant
                task_user_state = 2;
            }
            accept_users.accept_user_num += 1;
            accept_users.accept_user_names.push(person_name.clone());
            accept_users.accept_user_id.push(person_id.clone());
        }

        /*
        pub enum PartState {
            Accepted,
            Finished,
            Cancelled,
        }
        */
        // If finish mission
        if participant_state == 1 {
            if person_id == data.userid {
                // Set user state to finished participant
                task_user_state = 1;
            }
            finish_users.finish_user_num += 1;
            finish_users.finish_user_names.push(person_name.clone());
            finish_users.finish_user_id.push(person_id.clone());
        }
    }

    result_obj.accept_users = Some(accept_users);
    result_obj.finish_users = Some(finish_users);
    result_obj.task_user_state = Some(task_user_state);

    // Finish, Set Response Valid
    result_obj.code = true;

    HttpResponse::Ok().json(result_obj)
}

pub fn check_task_self_receive(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::GetTasksObj {
        code: true,
        err_message: "".to_string(),
        tasks: vec![],
    };

    // Init db-control
    let db_control = Controller::new();

    // Get target user's database-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let database_user_id = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(_)) => -1,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    if database_user_id == -1 {
        result_obj.code = false;
        result_obj.err_message = "Target user not exist Or Target user is a cow-user!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Get target student's take-part-in missions list
    let receive_tasks = db_control.get_student_missions(database_user_id);

    for task in receive_tasks.iter() {
        // Check target mission time state
        let over_time = task.deadline < (Local::now()).naive_local();

        let database_poster_id: UserId = UserId::Uid(task.poster_uid);
        let person_name = match db_control.get_user_from_identifier(database_poster_id) {
            Some(User::Student(stu)) => stu.username,
            Some(User::Cow(cow)) => cow.username,
            None => "".to_string(),
        };

        let mut user_finish_state = false;
        // Find student finish state
        for person in task.participants.iter() {
            if person.student_uid == database_user_id && person.state.to_val() == 1 {
                user_finish_state = true;
            }
        }

        let task_obj = json_objs::TaskBriefObj {
            mid: task.mid,
            poster_id: task.poster_uid,
            poster_name: person_name,
            task_state: over_time,
            task_name: task.name.clone(),
            task_intro: task.content.clone(),
            task_mode: task.mission_type.to_val().into(),
            task_pay: task.bounty,
            task_time_limit: task.deadline.to_string(),
            user_finish_state: Some(user_finish_state),
        };

        result_obj.tasks.push(task_obj);
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn check_task_self_release(data: web::Json<json_objs::UserIdObj>) -> HttpResponse {
    let mut result_obj = json_objs::GetTasksObj {
        code: true,
        err_message: "".to_string(),
        tasks: vec![],
    };

    // Init db-control
    let db_control = Controller::new();

    // Get target user's database-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let (database_user_id, username) = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(cow)) => (cow.uid, cow.username),
        Some(User::Student(stu)) => (stu.uid, stu.username),
        None => (-1, "".to_string()),
    };

    if database_user_id == -1 {
        result_obj.code = false;
        result_obj.err_message = "Target user not exist Or Target user is a cow-user!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Get target student's take-part-in missions list
    let receive_tasks = db_control.get_poster_missions(database_user_id);

    for task in receive_tasks.iter() {
        // Check target mission time state
        let over_time = task.deadline < (Local::now()).naive_local();

        let task_obj = json_objs::TaskBriefObj {
            mid: task.mid,
            poster_id: task.poster_uid,
            poster_name: username.clone(),
            task_state: over_time,
            task_name: task.name.clone(),
            task_intro: task.content.clone(),
            task_mode: task.mission_type.to_val().into(),
            task_pay: task.bounty,
            task_time_limit: task.deadline.to_string(),
            user_finish_state: None,
        };

        result_obj.tasks.push(task_obj);
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn check_question_naire(data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::AllAnswerObj {
        code: true,
        err_message: "".to_string(),
        answers: vec![],
    };

    let db_control = Controller::new();

    // Check target mission exist or not
    match db_control.get_mission_from_mid(data.task_mid) {
        Some(_) => {}
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find mission with target mid!".to_string();
            return HttpResponse::Ok().json(result_obj);
        }
    }

    // Judge target student is mission participant or not
    let mut is_part = false;
    let mission = db_control.get_mission_from_mid(data.task_mid).unwrap();
    for person in mission.participants.iter() {
        if person.student_uid == data.student_id {
            is_part = true;
            break;
        }
    }

    if !is_part {
        result_obj.code = false;
        result_obj.err_message = "Error! Targets student is not the mission's participant!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check user existence
    let user_wechat_id: UserId = UserId::WechatId(&data.userid);
    let cow_user_uid = match db_control.get_user_from_identifier(user_wechat_id) {
        Some(User::Cow(cow)) => cow.uid,
        Some(User::Student(_)) => -1,
        None => -1,
    };

    if cow_user_uid == -1 || cow_user_uid != mission.poster_uid {
        result_obj.code = false;
        result_obj.err_message =
            "Error! Only Questionnaire Poster Cow-User can check questionnaire answers!"
                .to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Get mission questionnaire
    let question_naire = match db_control.get_questionnaire(data.task_mid) {
        Ok(questions) => questions,
        Err(err) => {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
            vec![]
        }
    };

    if question_naire.len() == 0 {
        return HttpResponse::Ok().json(result_obj);
    } else {
        // Get target student answers
        let student_answers = match db_control.get_student_answer(data.task_mid, data.student_id) {
            Ok(answers) => Some(answers),
            Err(err) => {
                result_obj.code = false;
                result_obj.err_message = format!("{}", err);
                None
            }
        };

        if result_obj.code {
            let real_answers = student_answers.unwrap();
            let mut index = 0;
            for answer in real_answers.user_answer.into_iter() {
                let (q_type, string_answer, choose_answer) = match answer {
                    AnswerContent::SingleChoice(index) => (1, None, Some(vec![index])),
                    AnswerContent::MultiChoice(indexes) => (2, None, Some(indexes)),
                    AnswerContent::Blank(answer) => (0, Some(answer), None),
                };

                let the_answer = json_objs::AnswerObj {
                    order: index,
                    q_type: q_type,
                    content: question_naire[index as usize].description.clone(),
                    answer: string_answer,
                    choices: choose_answer,
                };

                result_obj.answers.push(the_answer);
                index += 1;
            }
        } else {
            return HttpResponse::Ok().json(result_obj);
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn check_transaction(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::TransactionResultObj {
        code: true,
        err_message: "".to_string(),
        t_type: "".to_string(),
        info: "".to_string(),
        loss: 0,
        address: "".to_string(),
    };

    let db_control = Controller::new();

    match db_control.get_trade(data.task_mid) {
        Some(trade) => {
            result_obj.t_type = trade.item_type.clone();
            result_obj.info = trade.item_info.clone();
            result_obj.loss = trade.item_condition;
            result_obj.address = trade.address.clone();
        }
        None => {
            result_obj.code = false;
            result_obj.err_message =
                "Error! Cannot find target transaction task with task_mid!".to_string();
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn check_errand(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::ErrandResultObj {
        code: true,
        err_message: "".to_string(),
        pickup_address: "".to_string(),
        deliver_address: "".to_string(),
        phone_number: "".to_string(),
        pick_number: "".to_string(),
        info: "".to_string(),
    };

    let db_control = Controller::new();

    match db_control.get_errand(data.task_mid) {
        Some(errand) => {
            result_obj.pickup_address = errand.pickup_address.clone();
            result_obj.info = errand.other_info.clone();
            result_obj.deliver_address = errand.deliver_address.clone();
            result_obj.phone_number = errand.phone_number.clone();
            result_obj.pick_number = match errand.item_code {
                Some(x) => x.clone(),
                None => "".to_string(),
            };
        }
        None => {
            result_obj.code = false;
            result_obj.err_message =
                "Error! Cannot find target errand task with task_mid!".to_string();
        }
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn receive_task(data: web::Json<json_objs::CheckTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    // Init DB Control
    let db_control = Controller::new();

    // Find target poster by poster_id
    let poster_database_id: UserId = UserId::Uid(data.poster_id);
    let has_poster = match db_control.get_user_from_identifier(poster_database_id) {
        Some(_) => true,
        None => false,
    };

    if !has_poster {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target poster in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Find student database-id by wechat-id
    let wechat_user_id: UserId = UserId::WechatId(&data.userid);
    let (database_user, find_stu) = match db_control.get_user_from_identifier(wechat_user_id) {
        Some(User::Cow(_cow)) => (None, false),
        Some(User::Student(stu)) => (Some(stu), true),
        None => (None, false),
    };

    if !find_stu {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot find target student user in database!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    let database_real_user = database_user.unwrap();
    // Check target participant is poster or not
    if database_real_user.uid == data.poster_id {
        result_obj.code = false;
        result_obj.err_message = "Error! Cannot join the mission posted by yourself!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    let task_mid = data.task_mid;

    let mut task_enable = true;

    match db_control.get_mission_from_mid(task_mid) {
        Some(task) => {
            if task.poster_uid != data.poster_id {
                task_enable = false;
                result_obj.err_message =
                    "Error! Target mission is not posted by target poster!".to_string();
            }
        }
        None => {
            task_enable = false;
            result_obj.err_message = "Error! Cannot find target mission in database!".to_string();
        }
    };

    if !task_enable {
        result_obj.code = false;
        return HttpResponse::Ok().json(result_obj);
    }

    if task_mid > 0 {
        // Check duplication
        let participants = db_control.get_mission_participants(task_mid);
        for person in participants.iter() {
            if person.student_uid == database_real_user.uid {
                result_obj.code = false;
                result_obj.err_message = "Error! Task Participant Duplication!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }

        let target_task = db_control.get_mission_from_mid(task_mid).unwrap();
        // Check student condition satisify
        let mut stu_condition = [false, false, false, false, false];
        // Check school condition
        stu_condition[0] = match target_task.school {
            Some(school) => database_real_user.school_id == school,
            None => true,
        };
        // Check min_finished condition
        stu_condition[1] = match target_task.min_finished {
            Some(require) => database_real_user.finished >= require,
            None => true,
        };
        // Check credit condition
        stu_condition[2] = match target_task.min_credit {
            Some(require) => database_real_user.credit >= require,
            None => true,
        };
        // Check major condition
        stu_condition[3] = match target_task.major {
            Some(require) => database_real_user.major == require,
            None => true,
        };
        // Check grade condition
        let min_grade = match target_task.min_grade {
            Some(min) => min,
            None => -1,
        };
        let max_grade = match target_task.max_grade {
            Some(max) => max,
            None => 5,
        };
        if database_real_user.year >= min_grade && database_real_user.year <= max_grade {
            stu_condition[4] = true;
        }

        for is_enable in stu_condition.iter() {
            if !is_enable {
                result_obj.code = false;
                result_obj.err_message =
                    "Receive Task Error! Student-User does not satisfy task request!".to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }

        // Check task exceed max_participants or not
        let mut find_mission = false;
        match db_control.get_mission_from_mid(task_mid) {
            Some(x) => {
                find_mission = true;
                result_obj.code = match x.max_participants {
                    Some(parts) => parts > (participants.len() as i32),
                    None => true,
                }
            }
            None => {
                result_obj.code = false;
                result_obj.err_message = "Error! Target Mission Not Exist!".to_string();
            }
        };

        if find_mission && !result_obj.code {
            result_obj.err_message = "Error! Target Task Exceed Max Participants Size!".to_string();
        }

        // Check participant's balance to support transaction
        if target_task.mission_type.to_val() == 1 {
            let user_balance = database_real_user.tokens;
            if target_task.bounty + user_balance < 0 {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Receiver does not have enough balance to support transaction!"
                        .to_string();
                return HttpResponse::Ok().json(result_obj);
            }
        }

        // Pass Checking, store participant into db
        if result_obj.code {
            let new_part_user = vec![Participant {
                student_uid: database_real_user.uid,
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

pub fn submit_task(data: web::Json<json_objs::SubmitTaskObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    // Check target mission existence and whether current user is the poster
    match db_control.get_mission_from_mid(data.task_mid) {
        Some(task) => {
            let current_wechat_id: UserId = UserId::WechatId(&data.userid);
            let current_uid = match db_control.get_user_from_identifier(current_wechat_id) {
                Some(User::Cow(cow)) => cow.uid,
                Some(User::Student(stu)) => stu.uid,
                None => -1,
            };

            if task.poster_uid != current_uid {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Current User does not have permission to confirm task submition!"
                        .to_string();
            }

            // Only transaction and errand task can use this API
            if task.mission_type.to_val() == 0 {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Only transaction and errand tasks can confirm by poster!".to_string();
            }
        }
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find target mission with task_mid!".to_string();
        }
    }

    if !result_obj.code {
        return HttpResponse::Ok().json(result_obj);
    }

    // Check target student is mission participant or not
    let task_participant = db_control.get_mission_participants(data.task_mid);

    let mut is_part = false;
    for person in task_participant.iter() {
        if person.student_uid == data.student_id {
            // Submit target student's mission state
            let updated_part = Participant {
                student_uid: data.student_id,
                state: PartState::Finished,
            };
            match db_control.update_participant(data.task_mid, &updated_part) {
                Ok(_) => {}
                Err(err) => {
                    result_obj.code = false;
                    result_obj.err_message = format!("{}", err);
                    return HttpResponse::Ok().json(result_obj);
                }
            }
            is_part = true;
            break;
        }
    }

    if !is_part {
        result_obj.code = false;
        result_obj.err_message = "Error! Target student is not mission's participant!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn submit_task_stu(data: web::Json<json_objs::SubmitQuestionNaireObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };

    let db_control = Controller::new();

    // Get target mission
    match db_control.get_mission_from_mid(data.task_mid) {
        Some(task) => {
            if data.poster_id != task.poster_uid {
                result_obj.code = false;
                result_obj.err_message =
                    "Error! Mission poster does not match your input poster_id!".to_string();
            }
        }
        None => {
            result_obj.code = false;
            result_obj.err_message = "Error! Cannot find target mission in database!".to_string();
        }
    }

    if !result_obj.code {
        return HttpResponse::Ok().json(result_obj);
    }

    // Get current user's database uid
    let current_uid = match db_control.get_user_from_identifier(UserId::WechatId(&data.userid)) {
        Some(User::Cow(_)) => -1,
        Some(User::Student(stu)) => stu.uid,
        None => -1,
    };

    // Judge current user is participant or not
    let participants = db_control.get_mission_participants(data.task_mid);
    let mut is_part = false;

    for person in participants.iter() {
        if person.student_uid == current_uid {
            // Submit target student's mission state
            let updated_part = Participant {
                student_uid: current_uid,
                state: PartState::Finished,
            };
            match db_control.update_participant(data.task_mid, &updated_part) {
                Ok(_) => {}
                Err(err) => {
                    result_obj.code = false;
                    result_obj.err_message = format!("{}", err);
                    return HttpResponse::Ok().json(result_obj);
                }
            }
            is_part = true;
            break;
        }
    }

    if !is_part {
        result_obj.code = false;
        result_obj.err_message =
            "Error! Current user is not target mission's participant!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Finally store question_answers into db
    let mut the_answer = Answer {
        mid: data.task_mid,
        user_id: current_uid,
        user_answer: vec![],
    };

    let target_questions = match db_control.get_questionnaire(data.task_mid) {
        Ok(questions) => questions,
        Err(err) => {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
            vec![]
        }
    };

    let mut index = 0;
    for answer in data.answers.iter() {
        let type_answer_matched = match target_questions[index as usize].choices {
            QuestionContent::SingleChoice(_) | QuestionContent::MultiChoice(_) => {
                match answer.choices {
                    Some(_) => true,
                    None => false,
                }
            }
            QuestionContent::Blank => match answer.answer {
                Some(_) => true,
                None => false,
            },
        };

        if !type_answer_matched {
            result_obj.code = false;
            result_obj.err_message =
                "Error! Question Type and Answer Type not matched!".to_string();
            break;
        }

        let special_answer = match target_questions[index as usize].choices {
            QuestionContent::SingleChoice(_) => {
                AnswerContent::SingleChoice(answer.choices.clone().unwrap()[0])
            }
            QuestionContent::MultiChoice(_) => {
                AnswerContent::MultiChoice(answer.choices.clone().unwrap())
            }
            QuestionContent::Blank => AnswerContent::Blank(answer.answer.clone().unwrap()),
        };

        the_answer.user_answer.push(special_answer);
        index += 1;
    }

    // Store into db
    match db_control.add_answer(&the_answer) {
        Ok(_) => {}
        Err(err) => {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    }    

    HttpResponse::Ok().json(result_obj)
}

pub fn get_tasks(data: web::Json<json_objs::TaskTypeObj>) -> HttpResponse {
    let mut result_obj = json_objs::GetTasksObj {
        code: true,
        err_message: "".to_string(),
        tasks: vec![],
    };

    let db_control = Controller::new();
    let mission_list = db_control.get_typed_mission_list(MissionType::from_val(data.task_type));

    for mission in mission_list.iter() {
        // Get poster name
        let database_user_id: UserId = UserId::Uid(mission.poster_uid);
        let poster_name = match db_control.get_user_from_identifier(database_user_id) {
            Some(User::Cow(cow)) => cow.username,
            Some(User::Student(stu)) => stu.username,
            None => "".to_string(),
        };

        if poster_name == "".to_string() {
            result_obj.code = false;
            result_obj.err_message = "Database Error! Cannot reach poster's name!".to_string();
            return HttpResponse::Ok().json(result_obj);
        }

        // Judge task state
        let task_state = mission.deadline <= (Local::now()).naive_local();

        let task = json_objs::TaskBriefObj {
            mid: mission.mid,
            poster_id: mission.poster_uid,
            poster_name: poster_name,
            task_state: task_state,
            task_name: mission.name.clone(),
            task_intro: mission.content.clone(),
            task_mode: mission.mission_type.to_val().into(),
            task_pay: mission.bounty,
            task_time_limit: mission.deadline.to_string(),
            user_finish_state: None,
        };

        result_obj.tasks.push(task);
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn get_tasks_by_range(data: web::Json<json_objs::TaskRangeObj>) -> HttpResponse {
    let mut result_obj = json_objs::GetTasksObj {
        code: true,
        err_message: "".to_string(),
        tasks: vec![],
    };

    let db_control = Controller::new();
    let (start, end) = (data.start, data.start + data.offset);

    for index in start..end {
        match db_control.get_mission_from_mid(index) {
            Some(mission) => {
                // Get poster name
                let database_user_id: UserId = UserId::Uid(mission.poster_uid);
                let poster_name = match db_control.get_user_from_identifier(database_user_id) {
                    Some(User::Cow(cow)) => cow.username,
                    Some(User::Student(stu)) => stu.username,
                    None => "".to_string(),
                };

                if poster_name == "".to_string() {
                    result_obj.code = false;
                    result_obj.err_message = "Database Error! Cannot reach poster's name!".to_string();
                    return HttpResponse::Ok().json(result_obj);
                }

                // Judge task state
                let task_state = mission.deadline <= (Local::now()).naive_local();

                let task = json_objs::TaskBriefObj {
                    mid: mission.mid,
                    poster_id: mission.poster_uid,
                    poster_name: poster_name,
                    task_state: task_state,
                    task_name: mission.name.clone(),
                    task_intro: mission.content.clone(),
                    task_mode: mission.mission_type.to_val().into(),
                    task_pay: mission.bounty,
                    task_time_limit: mission.deadline.to_string(),
                    user_finish_state: None,
                };

                result_obj.tasks.push(task);
            }
            None => {}
        }
    }

    HttpResponse::Ok().json(result_obj)
}