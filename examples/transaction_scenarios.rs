// This example demonstrates various transaction scenarios using the Webpay Plus client.
// It is an interactive command-line tool that requires manual user input to simulate
// the redirection flow of a real web application.
//
// Usage:
// cargo run --example transaction_scenarios -- <scenario>
//
// Available scenarios:
// - success: Simulates a successful payment.
// - rejected: Simulates a payment rejected by the user on the Webpay platform.
// - abort: Simulates a payment aborted by the user (e.g., closing the browser).
// - refund: Simulates a successful payment followed by a refund.

use std::env;
use webpay::client::{WebpayClient, Environment, Credentials};
use webpay::types::CreateRequest;
use webpay::webpay_plus::is_authorized;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example transaction_scenarios -- <scenario>");
        println!("Available scenarios: success, rejected, abort, refund");
        return;
    }

    let scenario = &args[1];

    // The WebpayClient is the main entry point to the library.
    // It requires the environment (Integration or Production) and credentials.
    // For this example, we use the integration environment and credentials.
    let wp = WebpayClient::new(
        Environment::Integration,
        Credentials {
            commerce_code: "597055555532".into(),
            api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(),
        },
    );

    // Based on the command-line argument, we run the corresponding scenario.
    match scenario.as_str() {
        "success" => handle_success(&wp).await,
        "rejected" => handle_rejected(&wp).await,
        "abort" => handle_abort(&wp).await,
        "refund" => handle_refund(&wp).await,
        _ => {
            println!("Invalid scenario: {}", scenario);
            println!("Available scenarios: success, rejected, abort, refund");
        }
    }
}

// Scenario 1: A successful transaction.
// The user completes the payment on the Webpay platform.
async fn handle_success(wp: &WebpayClient) {
    println!("\n--- Running Scenario: Successful Transaction ---");
    println!("This scenario simulates a user successfully completing a payment.");

    // 1. Create the transaction details.
    let req = CreateRequest {
        buy_order: "ORDER-SUCCESS-123".into(),
        session_id: "sess-success-456".into(),
        amount: 1500,
        return_url: "http://localhost:3000/webpay/return".into(), // URL where the user will be redirected after payment.
    };
    println!("\n[Step 1] Creating transaction with the following details:\n{:#?}", req);

    // 2. Call `wp_create` to get the URL for redirection.
    let created = wp.wp_create(&req).await.expect("Failed to create transaction");
    println!("\n[Step 2] Transaction created successfully. Response:\n{:#?}", created);
    println!("\n[Step 3] Please open the following URL in your browser to proceed with the payment:");
    println!("{}", created.url);

    // 3. Manually get the token from the user.
    // In a real web application, the user would be redirected back to your `return_url`,
    // and the token (`token_ws`) would be part of the POST request.
    println!("\n[Step 4] After completing the payment, you will be redirected to a blank page.");
    println!("   Please enter the 'token_ws' value from the form data of that page:");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    // 4. Commit the transaction using the received token.
    println!("\n[Step 5] Committing the transaction with token: {}", token_ws);
    let commit = wp.wp_commit(token_ws).await.expect("Failed to commit transaction");
    println!("\n[Step 6] Transaction committed. Response:\n{:#?}", commit);

    // 5. Check if the transaction was authorized.
    if is_authorized(&commit) {
        println!("\n[Result] Transaction successful! ✅");
        println!("   - Order: {}", commit.buy_order);
        println!("   - Amount: {}", commit.amount);
        println!("   - Authorization Code: {:?}", commit.authorization_code);
    } else {
        println!("\n[Result] Transaction was not authorized. ❌");
    }
}

// Scenario 2: A transaction rejected by the user.
// The user cancels the payment on the Webpay platform.
async fn handle_rejected(wp: &WebpayClient) {
    println!("\n--- Running Scenario: Rejected Transaction ---");
    println!("This scenario simulates a user rejecting a payment on the Webpay platform.");

    // 1. Create the transaction details.
    let req = CreateRequest {
        buy_order: "ORDER-REJECTED-123".into(),
        session_id: "sess-rejected-456".into(),
        amount: 2000,
        return_url: "http://localhost:3000/webpay/return".into(),
    };
    println!("\n[Step 1] Creating transaction with the following details:\n{:#?}", req);

    // 2. Call `wp_create` to get the URL for redirection.
    let created = wp.wp_create(&req).await.expect("Failed to create transaction");
    println!("\n[Step 2] Transaction created successfully. Response:\n{:#?}", created);
    println!("\n[Step 3] Please open the following URL and reject the payment:");
    println!("{}", created.url);

    // 3. Manually get the token from the user.
    println!("\n[Step 4] After rejecting the payment, enter the 'token_ws' from the form data:");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    // 4. Commit the transaction.
    println!("\n[Step 5] Committing the transaction with token: {}", token_ws);
    let commit = wp.wp_commit(token_ws).await.expect("Failed to commit transaction");
    println!("\n[Step 6] Transaction committed. Response:\n{:#?}", commit);

    // 5. Check if the transaction was authorized.
    if is_authorized(&commit) {
        println!("\n[Result] Transaction was unexpectedly authorized. This should not happen in a rejection scenario. 🤔");
    } else {
        println!("\n[Result] Transaction rejected as expected. ❌");
        println!("   - Status: {}", commit.status);
        println!("   - Response Code: {:?}", commit.response_code);
    }
}

// Scenario 3: A transaction aborted by the user.
// The user closes the browser or navigates away before completing the payment.
async fn handle_abort(wp: &WebpayClient) {
    println!("\n--- Running Scenario: Aborted Transaction ---");
    println!("This scenario simulates a user aborting a payment by closing the browser.");

    // 1. Create the transaction details.
    let req = CreateRequest {
        buy_order: "ORDER-ABORT-123".into(),
        session_id: "sess-abort-456".into(),
        amount: 2500,
        return_url: "http://localhost:3000/webpay/return".into(),
    };
    println!("\n[Step 1] Creating transaction with the following details:\n{:#?}", req);

    // 2. Call `wp_create` to get the URL for redirection.
    let created = wp.wp_create(&req).await.expect("Failed to create transaction");
    println!("\n[Step 2] Transaction created successfully. Response:\n{:#?}", created);
    println!("\n[Step 3] Please open the following URL, but instead of paying, close the tab or browser.");
    println!("{}", created.url);

    // 3. Explain the abortion flow.
    println!("\n[Step 4] When a user aborts, Webpay redirects them to the `return_url` with different parameters.");
    println!("   Instead of `token_ws`, you will receive:");
    println!("   - TBK_TOKEN");
    println!("   - TBK_ORDEN_COMPRA");
    println!("   - TBK_ID_SESION");
    println!("\n[Result] This example does not handle the `return_url` logic, but a real application must.");
    println!("   You should check for these parameters to identify an aborted transaction.");
}

// Scenario 4: A successful transaction followed by a refund.
async fn handle_refund(wp: &WebpayClient) {
    println!("\n--- Running Scenario: Refund Transaction ---");
    println!("This scenario simulates a successful payment followed by a partial refund.");

    // 1. Create and commit a successful transaction first.
    let req = CreateRequest {
        buy_order: "ORDER-REFUND-123".into(),
        session_id: "sess-refund-456".into(),
        amount: 3000,
        return_url: "http://localhost:3000/webpay/return".into(),
    };
    println!("\n[Step 1] Creating transaction with the following details:\n{:#?}", req);

    let created = wp.wp_create(&req).await.expect("Failed to create transaction");
    println!("\n[Step 2] Transaction created successfully. Response:\n{:#?}", created);
    println!("\n[Step 3] Please open the following URL and complete the payment:");
    println!("{}", created.url);

    println!("\n[Step 4] After completing the payment, enter the 'token_ws':");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    println!("\n[Step 5] Committing the transaction with token: {}", token_ws);
    let commit = wp.wp_commit(token_ws).await.expect("Failed to commit transaction");
    println!("\n[Step 6] Transaction committed. Response:\n{:#?}", commit);

    // 2. If the transaction was successful, proceed with the refund.
    if is_authorized(&commit) {
        println!("\n[Result] Transaction successful! ✅");
        println!("\n[Step 7] Now, proceeding with a partial refund...");
        let amount_to_refund = 500; // Can be a partial or full refund.
        println!("   - Amount to refund: {}", amount_to_refund);

        // 3. Call `wp_refund` with the token of the original transaction and the amount.
        let refund = wp.wp_refund(token_ws, amount_to_refund).await.expect("Failed to refund transaction");
        println!("\n[Step 8] Refund processed. Response:\n{:#?}", refund);

        // 4. Check the refund response.
        // A `response_code` of 0 indicates a successful refund.
        if refund.response_code == Some(0) {
            println!("\n[Result] Refund successful! 💰");
        } else {
            println!("\n[Result] Refund failed. ❌");
        }
    } else {
        println!("\n[Result] Transaction was not authorized, so it cannot be refunded. ❌");
    }
}