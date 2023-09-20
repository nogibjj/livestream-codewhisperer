use aws_sdk_s3 as s3;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

//This function calls the SDK for S3 to list the buckets and returns a vector of the buckets
async fn get_list_buckets() -> Vec<String> {
    let shared_config = aws_config::load_from_env().await;
    let client = s3::Client::new(&shared_config);
    let resp = client.list_buckets().send().await.unwrap();
    let mut buckets = Vec::new();
    for bucket in resp.buckets.unwrap_or_default() {
        //append each bucket name to the vector
        buckets.push(bucket.name.unwrap_or_default());
    }
    buckets
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let command = event.payload.command;
    let buckets = get_list_buckets().await;
    println!("{:?}", buckets);
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {}.", command),
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    run(service_fn(function_handler)).await
}
