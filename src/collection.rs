use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionItem {
    pub info: Info,
    #[serde(default)]
    pub http: Option<HttpConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub method: String,
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
    pub body: Option<HttpBody>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpBody {
    #[serde(rename = "type")]
    pub body_type: String,
    pub data: Option<serde_yaml::Value>,
}
