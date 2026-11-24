use reqwest::blocking::Client;
use serde_json::Value;
use crate::models_wrapper::error::ModelsWrapperError;

pub fn qwen_wrapper(client: Client, body: Value) -> Result<Value, ModelsWrapperError>{
    //let client = reqwest::blocking::Client::new();
    let response: serde_json::Value = client
        .post("http://127.0.0.1:8081/v1/chat/completions")
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;
    Ok(response)
}