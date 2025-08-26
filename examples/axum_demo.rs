// This example demonstrates how to integrate the webpay-rs client with the Axum web framework.
// It sets up a simple web server with two routes:
// - `/pay`: Initiates a new payment transaction.
// - `/webpay/return`: Handles the return from the Webpay platform after the user completes the payment process.
//
// To run this example:
// 1. Make sure you have a valid integration API key.
// 2. Set the `api_key` in the `Credentials` struct below.
// 3. Run `cargo run --example axum_demo`.
// 4. Open `http://127.0.0.1:3000/pay` in your browser.

use axum::{routing::{get, post}, Router, extract::Form, response::Html};
use serde::Deserialize;
use webpay::{client::{WebpayClient, Environment, Credentials}, types::CreateRequest};
use webpay::webpay_plus::is_authorized;

#[tokio::main]
async fn main() {
    // Initialize the WebpayClient with integration credentials.
    // Replace "YOUR_API_KEY_SECRET" with your actual integration API key.
    let wp = WebpayClient::new(
        Environment::Integration,
        Credentials {
            commerce_code: "597055555532".into(),      // Default integration commerce code
            api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(), // Use the default integration API key
        },
    );

    // Set up the Axum router with the payment routes.
    // We clone the WebpayClient for each route handler.
    let app = Router::new()
        .route("/pay", get({
            let wp = wp.clone();
            move || pay(wp.clone())
        }))
        .route("/webpay/return", post({
            let wp = wp.clone();
            move |form| webpay_return(wp.clone(), form)
        }));

    // Start the server.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on http://127.0.0.1:3000");
    println!("Visit http://127.0.0.1:3000/pay to start a transaction.");
    axum::serve(listener, app).await.unwrap();
}

// Handler for the `/pay` route.
// This function initiates a transaction with Webpay.
async fn pay(wp: WebpayClient) -> Html<String> {
    // 1. Define the transaction details.
    let req = CreateRequest {
        buy_order: "ORDER-AXUM-123".into(),
        session_id: "sess-axum-456".into(),
        amount: 1990,
        return_url: "http://127.0.0.1:3000/webpay/return".into(),
    };

    // 2. Call `wp_create` to get the URL and token for redirection.
    println!("Initiating transaction: {:#?}", req);
    let created = wp.wp_create(&req).await.expect("Failed to create transaction");
    println!("Transaction created: {:#?}", created);

    // 3. Generate HTML to auto-submit a form that redirects the user to the Webpay URL.
    // This is the standard way to redirect a user to the Webpay platform.
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Redirecting to Webpay...</title>
        </head>
        <body>
            <p>You are being redirected to Webpay to complete your payment.</p>
            <form id="webpay-form" action="{url}" method="POST">
                <input type="hidden" name="token_ws" value="{token}"/>
            </form>
            <script>
                // Automatically submit the form to redirect the user.
                document.getElementById('webpay-form').submit();
            </script>
        </body>
        </html>
    "#, url=created.url, token=created.token))
}

// Struct to deserialize the form data received from Webpay upon return.
#[derive(Deserialize, Debug)]
struct ReturnForm {
    // `token_ws` is present in successful or rejected transactions.
    token_ws: Option<String>,
    // These `TBK_*` fields are present if the user aborts the transaction.
    #[serde(rename = "TBK_TOKEN")]
    tbk_token: Option<String>,
    #[serde(rename = "TBK_ORDEN_COMPRA")]
    _tbk_orden_compra: Option<String>,
    #[serde(rename = "TBK_ID_SESION")]
    _tbk_id_sesion: Option<String>,
}

// Handler for the `/webpay/return` route.
// Webpay redirects the user here after they complete or cancel the payment.
async fn webpay_return(wp: WebpayClient, Form(f): Form<ReturnForm>) -> Html<String> {
    println!("Received return from Webpay: {:#?}", f);

    // Case 1: Successful or Rejected Transaction
    // Webpay sends `token_ws` when the user completes the flow (either successfully or by rejecting).
    if let Some(token) = f.token_ws {
        println!("Committing transaction with token: {}", token);
        let commit = wp.wp_commit(&token).await.expect("Failed to commit transaction");
        println!("Transaction committed: {:#?}", commit);

        // Use `is_authorized` to check if the payment was successful.
        if is_authorized(&commit) {
            // Payment was successful.
            // Here you should update your application's state (e.g., mark the order as paid).
            return Html(format!(
                "<h1>Payment Successful! ✅</h1><p>Your order <strong>{}</strong> for the amount of <strong>{}</strong> was paid successfully.</p><p>Authorization Code: <strong>{}</strong></p>",
                commit.buy_order, commit.amount, commit.authorization_code.as_deref().unwrap_or("N/A")
            ));
        } else {
            // Payment was rejected by the user.
            // Here you should handle the rejection (e.g., show a message to the user).
            return Html(format!(
                "<h1>Payment Rejected ❌</h1><p>Your payment was rejected.</p><p>Status: {}</p><p>Response Code: {:?}</p>",
                commit.status, commit.response_code
            ));
        }
    }

    // Case 2: Aborted Transaction
    // Webpay sends `TBK_TOKEN` if the user closes the payment window.
    if let Some(_tbk_token) = f.tbk_token {
        // The user aborted the payment.
        // Handle this case accordingly (e.g., show a message, update order status to "aborted").
        return Html("<h1>Payment Aborted</h1><p>You have aborted the payment process.</p>".into());
    }

    // Case 3: Invalid request
    // This should not happen in a normal flow.
    Html("<h1>Invalid Request</h1><p>No valid token received from Webpay.</p>".into())
}
