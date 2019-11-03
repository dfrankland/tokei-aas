mod schema;
mod tokei;

use self::schema::{State, SCHEMA};
use http::{header::HeaderValue, status::StatusCode};
use tide::{error::ResultExt, App, Context, EndpointResult};

async fn handle_graphql(mut cx: Context<State>) -> EndpointResult {
    let query: juniper::http::GraphQLRequest = cx.body_json().await.client_err()?;
    let state = cx.state();
    let graphql_response = query.execute_async(&SCHEMA, state).await;
    let status = if graphql_response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let mut response = tide::response::json(graphql_response);
    *response.status_mut() = status;
    let headers = response.headers_mut();
    headers.insert("Connection", HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str("*").unwrap(),
    );
    Ok(response)
}

async fn handle_cors(mut _cx: Context<State>) -> EndpointResult {
    let mut response = tide::response::json("");
    *response.status_mut() = StatusCode::NO_CONTENT;
    let headers = response.headers_mut();
    headers.insert("Connection", HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str("*").unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_str("*").unwrap(),
    );
    Ok(response)
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() {
    let mut app = App::with_state(State::default());
    app.at("/graphql").post(handle_graphql).options(handle_cors);
    app.serve("127.0.0.1:8000").await.unwrap()
}
