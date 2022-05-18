mod request_structure;

use aws_sdk_s3::Client;
use lambda_http::lambda_runtime;
use lambda_http::IntoResponse;
use lambda_http::Request;
use lambda_http::Response;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
 
#[tokio::main]
async fn main() -> Result<(), Error> {
    //
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
 
async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
 
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
 
    let struct_records: request_structure::Records = serde_json::from_value(event).unwrap();
 
    println!("event => {:#?}", struct_records);
    println!("event => {:#?}", struct_records.Records[0].s3.bucket.name);
 

    Ok(json!({ "message": format!("Hello, {}!", struct_records.Records[0].s3.bucket.name) }))
}
