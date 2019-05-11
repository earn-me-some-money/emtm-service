/*
* Router Mount APIs
*/
use actix_web::{web, HttpRequest, HttpResponse};

use crate::control::controller;

// Common APIs
pub fn index(req: HttpRequest) -> HttpResponse {
    controller::index()
}

pub fn logup(data: web::Json<LogupObj>) -> HttpResponse {
    controller::logup(data)
}

// Cow Users - APIs

// Student Users - APIs
