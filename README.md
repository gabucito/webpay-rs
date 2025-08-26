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

First, create a `WebpayClient` instance with your credentials and the desired environment.

```rust
use webpay::{client::{WebpayClient, Environment, Credentials}, types::CreateRequest};
use webpay::webpay_plus::is_authorized;

let client = WebpayClient::new(
    Environment::Integration, // Or Environment::Production
    Credentials {
        commerce_code: "597055555532".into(),
        api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(),
    },
);
```

### Creating a Transaction

To start a new transaction, use the `wp_create` method. This will return a URL where the user should be redirected to complete the payment.

```rust
let create_request = CreateRequest {
    buy_order: "ORDER-123".into(),
    session_id: "sess-1".into(),
    amount: 1000,
    return_url: "https://example.com/return".into(),
};

let created_transaction = client.wp_create(&create_request).await?;
println!("Redirect user to: {}", created_transaction.url);
```

### Committing a Transaction

After the user completes the payment process on Transbank's platform, they will be redirected back to the `return_url` you specified. The `token_ws` will be included in the POST data. Use this token to commit the transaction.

#### Successful Transaction

If the payment is successful, the `is_authorized` function will return `true`.

```rust
let token_ws = "the_token_returned_by_transbank";
let committed = client.wp_commit(token_ws).await?;

if is_authorized(&committed) {
    println!("Transaction successful! ✅");
    // Here you should update your application's state, e.g., mark the order as paid.
} else {
    println!("Transaction rejected! ❌");
    // The transaction was not authorized.
}
```

#### Rejected Transaction

If the user rejects the transaction on the Transbank platform, the commit response will indicate this, and `is_authorized` will be `false`.

#### Aborted Transaction

If the user aborts the transaction (e.g., by closing the browser), they will be redirected to the `return_url` with parameters like `TBK_TOKEN`, `TBK_ORDEN_COMPRA`, and `TBK_ID_SESION`. Your application should handle this case. The library does not currently have a specific helper for this, but you can detect it by checking for these parameters in the return request.

### Refunding a Transaction

You can refund a previously committed transaction using the `wp_refund` method. You will need the `token_ws` of the original transaction.

```rust
let token_ws = "the_token_of_the_original_transaction";
let amount_to_refund = 500; // Can be a partial or full refund

let refund = client.wp_refund(token_ws, amount_to_refund).await?;
println!("Refund response: {:?}", refund);
```

### Getting Transaction Status

To get the status of a transaction, use the `wp_status` method with the transaction's token.

```rust
let token_ws = "the_token_of_the_transaction";
let status = client.wp_status(token_ws).await?;
println!("Transaction status: {:?}", status);
```

## Examples

The `examples` directory contains the following examples:

*   `axum_demo`: A web server that demonstrates how to use the library with the Axum framework.
*   `transaction_scenarios`: A command-line application that demonstrates how to handle different transaction scenarios.

To run the examples, use the following commands:

```bash
# Run the Axum demo
cargo run --example axum_demo

# Run the transaction scenarios example
cargo run --example transaction_scenarios -- <scenario>
```

## Testing

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

MIT