/*
* Router Mount APIs
*/
use actix_web::{web, HttpRequest, HttpResponse};

use crate::control::controller;
use crate::control::json_objs;

// Common APIs
pub fn index(req: HttpRequest) -> HttpResponse {
    controller::index()
}

pub fn logup(data: web::Json<json_objs::LogupObj>) -> HttpResponse {
    controller::logup(data)
}

// Cow Users - APIs

// Student Users - APIs
