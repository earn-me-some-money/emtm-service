use serde_derive::{Deserialize, Serialize};

/*
* Part One. Request Json Struct
*/

// Cow-Logup Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CowLogupObj {
    // Common Values
    pub username: String,
    pub userid: String,
    pub wechat_ok: bool,
    pub email: String,
    pub phone: String,
    pub infos: String,
    // Cows Value
    pub organization: String,
}

// Student-Logup Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct StuLogupObj {
    // Common Values
    pub username: String,
    pub userid: String,
    pub wechat_ok: bool,
    pub email: String,
    pub phone: String,
    pub infos: String,
    // Student Values
    pub school_name: String,
    pub student_id: String,
    pub major: String,
    pub year: i32,
}

// Login Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginObj {
    pub userid: String,
    pub wechat_ok: bool,
}

// Task request Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequestObj {
    pub min_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub school: Option<String>,
    pub major: Option<String>,
    pub task_expe: Option<i32>,
    pub credit_score: Option<i32>,
    pub max_participants: Option<i32>,
}

// Release task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseTaskObj {
    pub userid: String,
    pub task_name: String,
    pub task_intro: String,
    pub task_mode: i8,
    pub task_request: TaskRequestObj,
    pub task_pay: i32,
    pub task_risk: i32,
    pub task_time_limit: String,
}

// Check tasks Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTypeObj {
    pub task_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdObj {
    pub userid: String,
}

// Check Special Task Request
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTaskObj {
    pub userid: String,
    pub poster_id: i32,
    pub task_mid: i32,
}

// Create group Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupObj {
    pub userid: String,
    pub group_name: String,
    pub max_limit: i64,
}

// Join group Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGroupObj {
    pub userid: String,
    pub group_name: String,
}

// Add friend Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct AddFriendObj {
    pub userid: String,
    pub friend_name: String,
}

// Receive task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveTaskObj {
    pub userid: String,
    pub target_userid: String,
    pub target_task: String,
}

// Submit task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTaskObj {
    pub userid: String,
    pub target_userid: String,
    pub target_task: String,
}

// Check credit Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCreditObj {
    pub userid: String,
}

// Recharge Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RechargeObj {
    pub userid: String,
    pub recharge_amount: String,
}

// Withdraw Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawObj {
    pub userid: String,
    pub withdraw_amount: String,
}

// Mission Search Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct MissionSearchObj {
    pub keyword: String,
}

// Verify User Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct UserVerifyObj {
    pub image_data: String,
    pub verify_mode: bool,
    pub user_id: String,
    pub organization: String,
}

// Get Wechat id Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWechatIdObj {
    pub appid: String,
    pub secret: String,
    pub code: String,
}

// Question Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionObj {
    pub order: i32,
    pub q_type: i32,
    pub content: String,
    pub choices: Option<Vec<String>>,
}

// Transaction Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionObj {
    pub mid: i32,
    pub t_type: String,
    pub info: String,
    pub loss: String,
    pub address: String,
}

// Errand Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrandObj {
    pub mid: i32,
    pub address: String,
    pub phone_number: String,
    pub pick_number: String,
    pub info: String,
}

// Question Naire Mission Detail Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionNaireObj {
    pub mid: i32,
    pub questions: Vec<QuestionObj>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StuAnswerObj {
    pub order: i32,
    pub answer: Option<String>,
    pub choices: Option<Vec<String>>,
}

// Student submit questionnaire Obj
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitQuestionNaireObj {
    pub userid: String,
    pub poster_id: i32,
    pub task_mid: i32,
    pub answers: Vec<StuAnswerObj>,
}

// User Verify Input
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyInfo {
    pub image_data: String,
    pub verify_mode: bool,
    pub user_id: String,
    pub organization: String,
}

// Cow-User New Info Request
#[derive(Debug, Serialize, Deserialize)]
pub struct CowEditInfoObj {
    pub userid: String,
    pub new_email: String,
    pub new_phone: String,
    pub new_infos: String,
}

// Stu-User New Info Request
#[derive(Debug, Serialize, Deserialize)]
pub struct StuEditInfoObj {
    pub userid: String,
    pub new_email: String,
    pub new_phone: String,
    pub new_infos: String,
    pub new_major: String,
    pub new_year: i32,
}

// =================================== //

/*
* Part Two. Response Json Struct
*/

// Origin Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginObj {
    pub code: bool,
    pub err_message: String,
}

// Login Result Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResultObj {
    pub code: bool,
    pub user_type: i32,
    pub err_message: String,
}

// Mission build Success Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct MissionOkObj {
    pub code: bool,
    pub err_message: String,
    pub mid: i32,
}

// Credit Score Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditScoreObj {
    pub code: bool,
    pub err_message: String,
    pub credit_score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchElementObj {
    pub mid: i32,
    pub name: String,
    pub content: String,
    pub poster_userid: String,
    pub time_limit: String,
    pub score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultObj {
    pub code: bool,
    pub err_message: String,
    pub search_result: Vec<SearchElementObj>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatIdResultObj {
    pub openid: String,
    pub errcode: i32,
    pub errmsg: String,
}

// Request Form
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestForm {
    pub appid: String,
    pub secret: String,
    pub js_code: String,
    pub grant_type: String,
}

// Response Form
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseForm {
    pub openid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
    pub session_key: Option<String>,
    pub unionid: Option<String>,
}

// Mission brief decription
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskBriefObj {
    pub mid: i32,
    pub poster_id: i32,
    pub poster_name: String,
    pub task_state: bool,
    pub task_name: String,
    pub task_intro: String,
    pub task_mode: i32,
    pub task_pay: i32,
    pub task_time_limit: String,
    // For user receive-task mode
    pub user_finish_state: Option<bool>,
}

// Target mission description
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTasksObj {
    pub code: bool,
    pub err_message: String,
    pub tasks: Vec<TaskBriefObj>,
}

// Mission Accepter description
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAccepterObj {
    pub accept_user_num: i32,
    pub accept_user_names: Vec<String>,
    pub accept_user_id: Vec<String>,
}

// Mission Finisher description
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFinisherObj {
    pub finish_user_num: i32,
    pub finish_user_names: Vec<String>,
    pub finish_user_id: Vec<String>,
}

// Mission Detailed description
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDetailObj {
    pub code: bool,
    pub err_message: String,
    // Brief description
    pub mid: Option<i32>,
    pub poster_id: Option<i32>,
    pub poster_name: Option<String>,
    pub task_state: Option<bool>,
    pub task_user_state: Option<bool>,
    pub task_name: Option<String>,
    pub task_intro: Option<String>,
    pub task_mode: Option<i32>,
    pub task_pay: Option<i32>,
    pub task_time_limit: Option<String>,
    // More infos
    pub task_risk: Option<i32>,
    pub task_request: Option<TaskRequestObj>,
    // Accepter and Finisher list
    pub accept_users: Option<TaskAccepterObj>,
    pub finish_users: Option<TaskFinisherObj>,
}

// Questionnaire Answer Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerObj {
    pub order: i32,
    pub q_type: i32,
    pub content: String,
    pub answer: Option<String>,
    pub choices: Option<Vec<String>>,
}

// Questionnaire Whole Answers Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct AllAnswerObj {
    pub code: bool,
    pub err_message: String,
    pub answers: Vec<AnswerObj>,
}

// User Balance return
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceObj {
    pub code: bool,
    pub err_message: String,
    pub balance: i32,
}

// Cow-User Info return
#[derive(Debug, Serialize, Deserialize)]
pub struct CowInfoObj {
    pub code: bool,
    pub err_message: String,

    pub username: String,
    pub email: String,
    pub phone: String,
    pub infos: String,
    pub organization: String,
}

// Student-User Info return
#[derive(Debug, Serialize, Deserialize)]
pub struct StuInfoObj {
    pub code: bool,
    pub err_message: String,

    pub username: String,
    pub email: String,
    pub phone: String,
    pub infos: String,
    pub school_name: String,
    pub student_id: String,
    pub major: String,
    pub year: i32,

    pub credit: i32,
    pub accepted: i32,
    pub finished: i32,
}
