use serde_derive::{Deserialize, Serialize};

/*
* Part One. Request Json Struct
*/

// Logup Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LogupObj {
    pub userid: String,
    pub wechat_ok: bool,
    pub logup_mode: bool,
    pub email: String,
    pub organization: String,
}

// Login Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginObj {
    pub userid: String,
    pub wechat_ok: bool,
    pub login_mode: bool,
}

// Release task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseTaskObj {
    pub userid: String,
    pub release_mode: bool,
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

/*
* Part Two. Response Json Struct
*/

// Origin Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginObj {
    pub code: bool,
    pub err_message: String,
}

// View Task Status Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskViewObj {
    pub code: bool,
    pub err_message: String,
    pub task_status: String,
}

// Credit Score Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditScoreObj {
    pub code: bool,
    pub err_message: String,
    pub credit_score: i64,
}
