use std::env;

use hyper::{
    body::{Buf},
    Body, Client as HyperClient, Method, Request, Uri,
};


use paris::{error, info};


use serenity::json::json;


use crate::models::UrlscanResponse;

pub fn check_url(msg: &str) -> Option<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    //TODO: Port Kt Link etector to Rust
    let split_string: Vec<_> = msg.split_ascii_whitespace().collect();
    for elem in split_string {
        let msg_search = elem.starts_with("http");
        if msg_search {
            result.push(String::from(elem));
        }
    }

    if result.len() > 0 {
        return Some(result);
    } else {
        return None;
    }
}

pub async fn clean_url(dirty_url: &str) -> Result<String, ()> {
    let client = HyperClient::new();
    let api_key =
        env::var("URLSCAN_API_KEY").expect("Environment variable not set for urlscan.io API key");
    let check_uri: Uri = match dirty_url.parse() {
        Ok(value) => value,
        Err(_) => return Err(()),
    };
    let base_uri: Uri = match "https://urlscan.io/api/v1/scan/".parse() {
        Ok(value) => value,
        Err(_) => return Err(()),
    };
    let _json_body = json!(
        {
            "url": check_uri.to_string(),
            "visibility": "public"
        }
    );
    let json_body = match _json_body.as_str() {
        Some(val) => String::from(val),
        _ => return Err(()),
    };

    let req_result = Request::builder()
        .method(Method::POST)
        .uri(base_uri)
        .header("API KEY", api_key)
        .header("content-type", "application/json")
        .body(Body::from(json_body));
    let req = match req_result {
        Ok(val) => val,
        _ => return Err(()),
    };
    let resp_result = client.request(req).await;
    let mut resp = match resp_result {
        Ok(val) => val,
        _ => return Err(()),
    };
    info!("Status for submission of url {}", &resp.status());

    let body_reader_result = hyper::body::aggregate(resp.body_mut()).await;
    let body_reader = match body_reader_result {
        Ok(val) => val,
        _ => return Err(()),
    };
    let urlscan_resp_result: Result<UrlscanResponse, _> =
        serde_json::from_reader(body_reader.reader());
    match urlscan_resp_result {
        Ok(val) => return Ok(val.result),
        _ => return Err(()),
    };
}