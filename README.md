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

```rust
use webpay::{client::{WebpayClient, Environment, Credentials}, types::CreateRequest};
use webpay::webpay_plus::is_authorized;

let client = WebpayClient::new(
    Environment::Integration,
    Credentials {
        commerce_code: "597055555532".into(),
        api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(),
    },
);

let created = client.wp_create(&CreateRequest {
    buy_order: "ORDER-123".into(),
    session_id: "sess-1".into(),
    amount: 1000,
    return_url: "https://example.com/return".into(),
}).await?;
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