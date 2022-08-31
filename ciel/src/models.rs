use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct UrlscanResponse {
    pub message: String,
    pub uuid: String,
    pub result: String,
    pub api: String,
    pub visibility: String,
    pub options: String,
    pub url: String,
    pub country: String,
}