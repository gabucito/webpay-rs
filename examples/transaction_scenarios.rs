// Run: cargo run --example transaction_scenarios -- <scenario>
//
// Scenarios:
// - success: A successful transaction.
// - rejected: A transaction rejected by the user.
// - abort: A transaction aborted by the user.
// - refund: A successful transaction followed by a refund.

use std::env;
use webpay::client::{WebpayClient, Environment, Credentials};
use webpay::types::CreateRequest;
use webpay::webpay_plus::is_authorized;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example transaction_scenarios -- <scenario>");
        println!("Scenarios: success, rejected, abort, refund");
        return;
    }

    let scenario = &args[1];

    let wp = WebpayClient::new(
        Environment::Integration,
        Credentials {
            commerce_code: "597055555532".into(),
            api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(),
        },
    );

    match scenario.as_str() {
        "success" => handle_success(&wp).await,
        "rejected" => handle_rejected(&wp).await,
        "abort" => handle_abort(&wp).await,
        "refund" => handle_refund(&wp).await,
        _ => {
            println!("Invalid scenario: {}", scenario);
            println!("Scenarios: success, rejected, abort, refund");
        }
    }
}

async fn handle_success(wp: &WebpayClient) {
    println!("--- Running success scenario ---");
    let req = CreateRequest {
        buy_order: "ORDER-SUCCESS".into(),
        session_id: "sess-success".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = wp.wp_create(&req).await.expect("create");
    println!("Transaction created: {:?}", created);

    println!("Please go to the following URL to continue the transaction:");
    println!("{}", created.url);

    // In a real application, you would redirect the user to the URL above.
    // For this example, we will manually get the token from the user.
    println!("Please enter the token_ws from the URL:");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    let commit = wp.wp_commit(token_ws).await.expect("commit");
    println!("Transaction committed: {:?}", commit);

    if is_authorized(&commit) {
        println!("Transaction successful! ✅");
    } else {
        println!("Transaction rejected! ❌");
    }
}

async fn handle_rejected(wp: &WebpayClient) {
    println!("--- Running rejected scenario ---");
    let req = CreateRequest {
        buy_order: "ORDER-REJECTED".into(),
        session_id: "sess-rejected".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = wp.wp_create(&req).await.expect("create");
    println!("Transaction created: {:?}", created);

    println!("Please go to the following URL to continue the transaction and reject it:");
    println!("{}", created.url);

    // In a real application, you would redirect the user to the URL above.
    // For this example, we will manually get the token from the user.
    println!("Please enter the token_ws from the URL:");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    let commit = wp.wp_commit(token_ws).await.expect("commit");
    println!("Transaction committed: {:?}", commit);

    if is_authorized(&commit) {
        println!("Transaction successful! ✅");
    } else {
        println!("Transaction rejected! ❌");
    }
}

async fn handle_abort(wp: &WebpayClient) {
    println!("--- Running abort scenario ---");
    let req = CreateRequest {
        buy_order: "ORDER-ABORT".into(),
        session_id: "sess-abort".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = wp.wp_create(&req).await.expect("create");
    println!("Transaction created: {:?}", created);

    println!("Please go to the following URL to continue the transaction and abort it:");
    println!("{}", created.url);

    println!("After aborting the transaction, you will be redirected to the return_url.");
    println!("The return_url will contain the TBK_TOKEN, TBK_ORDEN_COMPRA, and TBK_ID_SESION parameters.");
    println!("This example does not handle the return URL, but in a real application, you would need to.");
}

async fn handle_refund(wp: &WebpayClient) {
    println!("--- Running refund scenario ---");
    let req = CreateRequest {
        buy_order: "ORDER-REFUND".into(),
        session_id: "sess-refund".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = wp.wp_create(&req).await.expect("create");
    println!("Transaction created: {:?}", created);

    println!("Please go to the following URL to continue the transaction:");
    println!("{}", created.url);

    // In a real application, you would redirect the user to the URL above.
    // For this example, we will manually get the token from the user.
    println!("Please enter the token_ws from the URL:");
    let mut token_ws = String::new();
    std::io::stdin().read_line(&mut token_ws).unwrap();
    let token_ws = token_ws.trim();

    let commit = wp.wp_commit(token_ws).await.expect("commit");
    println!("Transaction committed: {:?}", commit);

    if is_authorized(&commit) {
        println!("Transaction successful! ✅");
        println!("Refunding transaction...");
        let refund = wp.wp_refund(token_ws, 500).await.expect("refund");
        println!("Refund response: {:?}", refund);
    } else {
        println!("Transaction rejected! ❌");
    }
}