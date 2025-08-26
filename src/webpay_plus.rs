use crate::client::WebpayClient;
use crate::types::*;

static V1: &str = "/rswebpaytransaction/api/webpay/v1.2";

impl WebpayClient {
    /// Create a Webpay Plus transaction.
    ///
    /// This is the first step in the transaction flow. It returns a token and a URL that the user should be redirected to.
    ///
    /// # Arguments
    ///
    /// * `req` - A `CreateRequest` struct with the transaction details.
    pub async fn wp_create(&self, req: &CreateRequest) -> Result<CreateResponse, WebpayError> {
        let url = self.endpoint(&format!("{}/transactions", V1));
        let res = self.http()
            .post(url)
            .headers(self.headers_ref())
            .json(req)
            .send().await?;

        let status = res.status();
        if status.is_success() {
            Ok(res.json::<CreateResponse>().await?)
        } else {
            let body = res.text().await.unwrap_or_default();
            Err(WebpayError::Api(format!("create failed: {} {}", status, body)))
        }
    }

    /// Commit (confirm) a Webpay Plus transaction.
    ///
    /// This is the second step in the transaction flow. It should be called after the user is redirected back to the merchant's site.
    ///
    /// # Arguments
    ///
    /// * `token_ws` - The token received in the `CreateResponse`.
    pub async fn wp_commit(&self, token_ws: &str) -> Result<CommitResponse, WebpayError> {
        let url = self.endpoint(&format!("{}/transactions/{}", V1, token_ws));
        let res = self.http()
            .put(url)
            .headers(self.headers_ref())
            .send().await?;

        let status = res.status();
        if status.is_success() {
            Ok(res.json::<CommitResponse>().await?)
        } else {
            let body = res.text().await.unwrap_or_default();
            Err(WebpayError::Api(format!("commit failed: {} {}", status, body)))
        }
    }

    /// Get the status of a Webpay Plus transaction.
    ///
    /// # Arguments
    ///
    /// * `token_ws` - The token received in the `CreateResponse`.
    pub async fn wp_status(&self, token_ws: &str) -> Result<StatusResponse, WebpayError> {
        let url = self.endpoint(&format!("{}/transactions/{}", V1, token_ws));
        let res = self.http()
            .get(url)
            .headers(self.headers_ref())
            .send().await?;

        let status = res.status();
        if status.is_success() {
            Ok(res.json::<StatusResponse>().await?)
        } else {
            let body = res.text().await.unwrap_or_default();
            Err(WebpayError::Api(format!("status failed: {} {}", status, body)))
        }
    }

    /// Refund a Webpay Plus transaction.
    ///
    /// # Arguments
    ///
    /// * `token_ws` - The token of the transaction to refund.
    /// * `amount` - The amount to refund.
    pub async fn wp_refund(&self, token_ws: &str, amount: i64) -> Result<RefundResponse, WebpayError> {
        let url = self.endpoint(&format!("{}/transactions/{}/refunds", V1, token_ws));
        let req = RefundRequest { amount };
        let res = self.http()
            .post(url)
            .headers(self.headers_ref())
            .json(&req)
            .send().await?;

        let status = res.status();
        if status.is_success() {
            Ok(res.json::<RefundResponse>().await?)
        } else {
            let body = res.text().await.unwrap_or_default();
            Err(WebpayError::Api(format!("refund failed: {} {}", status, body)))
        }
    }
}

/// Helper to check if a transaction was successful.
///
/// A transaction is successful if `response_code` is `Some(0)` and `status` is `"AUTHORIZED"`.
pub fn is_authorized(r: &crate::types::CommitResponse) -> bool {
    matches!(r.response_code, Some(0)) && r.status == "AUTHORIZED"
}
