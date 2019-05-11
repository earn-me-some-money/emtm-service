use serde_derive::{Deserialize, Serialize};

// Origin Return Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginObj {
    code: bool,
    errMessage: String,
}

// Logup Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LogupObj {
    username: String,
    wechat_ok: bool,
    logup_mode: bool,
    email: String,
    organization: String,
}

// Login Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginObj {
    username: String,
    wechat_ok: bool,
    login_mode: bool,
}

// Release task Parse Json Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseTaskObj {
    username: String,
    release_mode: bool,
}
