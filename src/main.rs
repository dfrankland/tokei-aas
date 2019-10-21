mod schema;

use crate::schema::{SCHEMA, State};
use http::{status::StatusCode, header::HeaderValue};
use tide::{error::ResultExt, response, App, Context, EndpointResult};

async fn handle_graphql(mut cx: Context<State>) -> EndpointResult {
    let query: juniper::http::GraphQLRequest = cx.body_json().await.client_err()?;
    let response = query.execute(&SCHEMA, cx.state());
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let mut resp = response::json(response);
    *resp.status_mut() = status;
    let headers = resp.headers_mut();
    headers.insert("Connection", HeaderValue::from_str("keep-alive").unwrap());
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_str("*").unwrap());
    Ok(resp)
}

async fn handle_cors(mut _cx: Context<State>) -> EndpointResult {
    let mut resp = response::json("");
    *resp.status_mut() = StatusCode::NO_CONTENT;
    let headers = resp.headers_mut();
    headers.insert("Connection", HeaderValue::from_str("keep-alive").unwrap());
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_str("*").unwrap());
    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_str("*").unwrap());
    Ok(resp)
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() {
    let mut app = App::with_state(State::default());
    app.at("/graphql").post(handle_graphql).options(handle_cors);
    app.serve("127.0.0.1:8000").await.unwrap()
}
