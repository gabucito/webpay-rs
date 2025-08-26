# webpay-rs (Unofficial)

Async Rust client for **Transbank Webpay Plus REST**: `create`, `commit`, `status`, `refund`.  
Designed to be framework-agnostic; example shows Axum integration.

> ⚠️ This is **unofficial** and provided as a starting point. Always verify against the official Transbank docs and your commerce configuration.

## Features

*   ✅ Async client for Transbank Webpay Plus REST.
*   ✅ Create, commit, status, and refund transactions.
*   ✅ Framework-agnostic.
*   ✅ Configurable timeout for network requests.
*   ✅ Integration tests.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
webpay = { git = "https://github.com/gabucito/webpay-rs.git" }
```

## Usage

The basic workflow for a Webpay transaction involves the following steps:

1.  **Create a `WebpayClient`**: Initialize the client with your credentials and the desired environment (`Integration` or `Production`).
2.  **Create a Transaction**: Use `wp_create` to register the transaction with Transbank and get a URL to redirect the user.
3.  **Commit a Transaction**: After the user is redirected back to your site, use `wp_commit` with the received `token_ws` to confirm the payment.
4.  **Handle the Result**: Check if the transaction was authorized and update your application state accordingly.

```rust
use webpay::{client::{WebpayClient, Environment, Credentials}, types::CreateRequest};
use webpay::webpay_plus::is_authorized;

// 1. Initialize the client
let client = WebpayClient::new(
    Environment::Integration, // Use Environment::Production for live transactions
    Credentials {
        commerce_code: "597055555532".into(), // Your commerce code
        api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(), // Your secret API key
    },
);

// 2. Create the transaction
let create_request = CreateRequest {
    buy_order: "ORDER-123".into(),
    session_id: "sess-1".into(),
    amount: 1000,
    // For production, this must be a publicly accessible URL (e.g., https://your-site.com/webpay/return).
    // For local development, you can use a localhost URL (e.g., http://localhost:3000/webpay/return).
    return_url: "https://your-site.com/webpay-return".into(),
};
let created = client.wp_create(&create_request).await?;

// `created.url` is where you should redirect the user to complete the payment.
// `created.token` is the `token_ws` that will be used to identify the transaction.
println!("Redirect user to: {}?token_ws={}", created.url, created.token);

// 3. Commit the transaction (after user returns from Webpay)
// The `token_ws` is typically received as a POST parameter to your `return_url`.
let token_ws = "the_token_returned_by_transbank";
let committed = client.wp_commit(token_ws).await?;

// 4. Handle the result
if is_authorized(&committed) {
    println!("✅ Transaction successful!");
    // Mark the order as paid in your database.
} else {
    println!("❌ Transaction rejected or failed.");
    // The transaction was not authorized. Check `committed.status` and `committed.response_code`.
}
```

### Other Operations

#### Refunding a Transaction

You can refund a previously committed transaction. You'll need the `token_ws` of the original transaction.

```rust
let token_ws = "token_of_the_original_transaction";
let amount_to_refund = 500; // Can be a partial or full refund

let refund = client.wp_refund(token_ws, amount_to_refund).await?;
if refund.response_code == Some(0) {
    println!("💰 Refund successful!");
}
```

#### Getting Transaction Status

Check the status of any transaction using its `token_ws`.

```rust
let token_ws = "token_of_the_transaction_to_check";
let status = client.wp_status(token_ws).await?;
println!("Transaction status: {:?}", status);
```

## Detailed Examples

The `examples` directory contains fully commented, runnable examples that demonstrate common workflows. **It is highly recommended to review them.**

*   `axum_demo`: A complete web server that shows how to handle the entire payment flow, including creating, committing, and handling different return scenarios (success, rejection, abortion).
*   `transaction_scenarios`: An interactive command-line application to simulate and understand different outcomes like successful payments, rejections, and refunds.

### How to Run the Examples

#### Axum Web Server Demo

This example starts a web server on `http://127.0.0.1:3000`.

```bash
# Run the Axum demo
cargo run --example axum_demo
```

Open `http://127.0.0.1:3000/pay` in your browser to initiate a payment. The source code (`examples/axum_demo.rs`) is heavily commented to explain each step of the integration.

#### Interactive Transaction Scenarios

This example runs different scenarios in your terminal.

```bash
# Run the 'success' scenario
cargo run --example transaction_scenarios -- success

# Run the 'refund' scenario
cargo run --example transaction_scenarios -- refund
```

Available scenarios: `success`, `rejected`, `abort`, `refund`. The source code (`examples/transaction_scenarios.rs`) explains what happens in each case.

## Best Practices

When moving to a production environment, consider the following:

*   **Use Production Environment**: Change `Environment::Integration` to `Environment::Production` in the `WebpayClient` constructor. You will need to have valid production credentials from Transbank.

*   **Secure Credential Management**: Avoid hardcoding your `commerce_code` and `api_key`. Use environment variables, a `.env` file (with a library like `dotenv`), or a secret management service to keep your credentials secure.

    ```rust
    // Example using environment variables
    let api_key = std::env::var("WEBPAY_API_KEY").expect("WEBPAY_API_KEY must be set");
    let commerce_code = std::env::var("WEBPAY_COMMERCE_CODE").expect("WEBPAY_COMMERCE_CODE must be set");

    let credentials = Credentials {
        api_key: api_key.into(),
        commerce_code: commerce_code.into(),
    };
    ```

*   **Idempotency and State Management**: When handling the return from Webpay, ensure your logic for updating your database (e.g., marking an order as paid) is idempotent. This means that if the same successful transaction notification is processed multiple times, it does not result in duplicate updates. Always verify the transaction status with `wp_commit` before updating your system.

## Testing

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

MIT