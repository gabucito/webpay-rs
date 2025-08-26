use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum WebpayError {
    #[error("http {0}")]
    Http(#[from] reqwest::Error),
    #[error("webpay error: {0}")]
    Api(String),
    #[error("unexpected response")]
    Unexpected,
}

//
// Create
//
#[derive(Debug, Serialize)]
pub struct CreateRequest {
    /// Up to 26 chars
    pub buy_order: String,
    pub session_id: String,
    /// Amount in CLP (integer)
    pub amount: i64,
    /// Your return endpoint; Webpay will POST back here with token_ws
    pub return_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub token: String,
    pub url: String, // redirect target to POST token_ws
}

//
// Commit / Status responses
//
#[derive(Debug, Deserialize)]
pub struct CardDetail {
    pub card_number: Option<String>, // last 4 digits
}

#[derive(Debug, Deserialize)]
pub struct CommitResponse {
    pub vci: Option<String>,
    pub amount: i64,
    pub status: String, // "AUTHORIZED" on success
    pub buy_order: String,
    pub session_id: String,
    pub card_detail: Option<CardDetail>,
    pub accounting_date: Option<String>,   // e.g., "0522"
    pub transaction_date: Option<DateTime<Utc>>,
    pub authorization_code: Option<String>,// e.g., "1213"
    pub payment_type_code: Option<String>, // "VN","VD","VC","SI","S2","NC"
    pub response_code: Option<i32>,        // success == 0
    pub installments_number: Option<i32>,
    pub installments_amount: Option<i64>,
}

pub type StatusResponse = CommitResponse;

//
// Refund
//
#[derive(Debug, Serialize)]
pub struct RefundRequest {
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct RefundResponse {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_date: Option<DateTime<Utc>>,
    pub nullified_amount: Option<i64>,
    pub balance: Option<i64>,
    pub response_code: Option<i32>, // 0 on success
}
