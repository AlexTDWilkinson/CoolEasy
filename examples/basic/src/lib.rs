use cool_easy::{html, md, PageWrapper};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use validator::{Validate, ValidationError};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

// use crate::CoolEasyValidationError;

#[derive(Debug)]
pub struct CoolEasyValidationError {
    pub field: String,
    pub message: String,
}
