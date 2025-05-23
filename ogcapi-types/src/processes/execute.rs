use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::common::{Bbox, Link, OGC_CRS84};

/// Process execution
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Execute {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, Input>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub outputs: HashMap<String, Output>,
    #[serde(default)]
    pub response: Response,
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
}

/// Process execution input
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Input {
    InlineOrRefData(InlineOrRefData),
    InlineOrRefDataArray(Vec<InlineOrRefData>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InlineOrRefData {
    InputValueNoObject(InputValueNoObject),
    QualifiedInputValue(QualifiedInputValue),
    Link(Link),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputValueNoObject {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Array(Vec<Value>),
    // TODO: requires custom serde implementation
    // BinaryInputValue(String), // Undistinguishable from String(String)
    Bbox(BoundingBox), // bbox is actually an object
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoundingBox {
    pub bbox: Bbox,
    #[serde(default = "default_crs")]
    pub crs: String,
}

fn default_crs() -> String {
    OGC_CRS84.to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualifiedInputValue {
    pub value: InputValue,
    #[serde(flatten)]
    pub format: Format,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputValue {
    InputValueNoObject(InputValueNoObject),
    Object(Map<String, Value>),
}

/// Process execution output
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub format: Option<Format>,
    pub transmission_mode: TransmissionMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub media_type: Option<String>,
    pub encoding: Option<String>,
    pub schema: Option<Schema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Schema {
    String(String),
    Object(Map<String, Value>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum TransmissionMode {
    #[default]
    Value,
    Reference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Response {
    #[default]
    Raw,
    Document,
}

/// Process execution subscriber
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub success_uri: String,
    pub in_progress_uri: Option<String>,
    pub failed_uri: Option<String>,
}
