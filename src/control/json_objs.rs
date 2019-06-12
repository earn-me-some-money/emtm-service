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
    pub login_mode: bool,
}

// Task request Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequestObj {
    pub grade: Option<i8>,
    pub major: Option<String>,
    pub task_exper: Option<i32>,
    pub credit_score: Option<i32>,
    pub max_participants: i32,
}

// Release task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseTaskObj {
    pub userid: String,
    pub release_mode: bool,
    pub task_name: String,
    pub task_intro: String,
    pub task_mode: i8,
    pub task_request: TaskRequestObj,
    pub task_pay: i32,
    pub task_risk: i32,
    pub task_time_limit: String,
}

// Check task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTaskObj {
    pub userid: String,
    pub task_name: String,
    pub check_mode: bool,
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
    pub receive_mode: bool,
    pub target_userid: String,
    pub target_task: String,
}

// Submit task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTaskObj {
    pub userid: String,
    pub submit_mode: bool,
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

/*
* Part Two. Response Json Struct
*/

// Origin Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginObj {
    pub code: bool,
    pub err_message: String,
}

// Student Finish Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct StuTaskStatusObj {
    pub student_userid: String,
    pub is_finish: bool,
}

// View Task Status Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskViewObj {
    pub code: bool,
    pub err_message: String,
    pub task_state: String,
    pub task_status: Vec<StuTaskStatusObj>,
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
