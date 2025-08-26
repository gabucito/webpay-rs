use webpay::client::{WebpayClient, Environment, Credentials};
use webpay::types::CreateRequest;

fn get_client() -> WebpayClient {
    WebpayClient::new(
        Environment::Integration,
        Credentials {
            commerce_code: "597055555532".into(),
            api_key: "579B532A7440BB0C9079DED94D31EA1615BACEB56610332264630D42D0A36B1C".into(),
        },
    )
}

#[tokio::test]
async fn test_create_transaction() {
    let client = get_client();
    let req = CreateRequest {
        buy_order: "ORDER-TEST-CREATE".into(),
        session_id: "sess-test-create".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let res = client.wp_create(&req).await;
    assert!(res.is_ok());

    let created = res.unwrap();
    assert!(!created.token.is_empty());
    assert!(!created.url.is_empty());
}

#[tokio::test]
async fn test_commit_transaction() {
    let client = get_client();
    let req = CreateRequest {
        buy_order: "ORDER-TEST-COMMIT".into(),
        session_id: "sess-test-commit".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = client.wp_create(&req).await.expect("create");

    // In a real application, you would redirect the user to the URL and get the token back.
    // For this test, we can't do that, so we will just check the status of the transaction.
    let status = client.wp_status(&created.token).await.expect("status");

    // The status of a newly created transaction should be "INITIALIZED".
    assert_eq!(status.status, "INITIALIZED");
}

#[tokio::test]
async fn test_refund_transaction() {
    let client = get_client();
    let req = CreateRequest {
        buy_order: "ORDER-TEST-REFUND".into(),
        session_id: "sess-test-refund".into(),
        amount: 1000,
        return_url: "http://localhost:3000/return".into(),
    };

    let created = client.wp_create(&req).await.expect("create");

    // To refund a transaction, it must be committed first.
    // We can't do that in this test, so we will just check that the refund endpoint returns an error.
    let refund = client.wp_refund(&created.token, 500).await;
    assert!(refund.is_err());
}