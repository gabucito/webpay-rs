// Run: cargo run --example axum_demo
use axum::{routing::{get, post}, Router, extract::Form, response::Html};
use serde::Deserialize;
use webpay::{client::{WebpayClient, Environment, Credentials}, types::CreateRequest};
use webpay::webpay_plus::is_authorized;

#[tokio::main]
async fn main() {
    let wp = WebpayClient::new(
        Environment::Integration,
        Credentials {
            commerce_code: "597055555532".into(),      // Example integration commerce code
            api_key: "YOUR_API_KEY_SECRET".into(),     // Put your integration secret
        },
    );

    // store client in state
    let app = Router::new()
        .route("/pay", get({
            let wp = wp.clone();
            move || pay(wp.clone())
        }))
        .route("/webpay/return", post({
            let wp = wp.clone();
            move |form| webpay_return(wp.clone(), form)
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn pay(wp: WebpayClient) -> Html<String> {
    let req = CreateRequest {
        buy_order: "ORDER-123456".into(),
        session_id: "sess-abc-123".into(),
        amount: 12345,
        return_url: "http://127.0.0.1:3000/webpay/return".into(),
    };
    let created = wp.wp_create(&req).await.expect("create");
    Html(format!(r#"
        <form id="wp" action="{url}" method="POST">
          <input type="hidden" name="token_ws" value="{token}"/>
        </form>
        <script>document.getElementById('wp').submit();</script>
    "#, url=created.url, token=created.token))
}

#[derive(Deserialize)]
struct ReturnForm {
    token_ws: Option<String>,
    #[serde(rename = "TBK_TOKEN")]
    tbk_token: Option<String>,       // when user aborts
    #[serde(rename = "TBK_ORDEN_COMPRA")]
    _tbk_orden_compra: Option<String>,
    #[serde(rename = "TBK_ID_SESION")]
    _tbk_id_sesion: Option<String>,
}

async fn webpay_return(wp: WebpayClient, Form(f): Form<ReturnForm>) -> Html<String> {
    if let Some(token) = f.token_ws {
        let commit = wp.wp_commit(&token).await.expect("commit");
        if is_authorized(&commit) {
            return Html(format!("OK ✅ order={} amount={} auth={:?}",
                commit.buy_order, commit.amount, commit.authorization_code));
        } else {
            return Html(format!("Rejected ❌ code={:?} status={}", commit.response_code, commit.status));
        }
    }

    // Aborted / timeout flows
    if let Some(_tbk_token) = f.tbk_token {
        return Html("Aborted".into());
    }

    Html("No token received".into())
}
