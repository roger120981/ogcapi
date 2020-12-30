use super::ContentType;
use serde::{Deserialize, Serialize};

/// Hyperlink to enable Hypermedia Access
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Link {
    pub href: String,
    pub rel: LinkRelation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ContentType>>,
}

/// Link Relations
///
/// [IANA Link Relations Registry](https://www.iana.org/assignments/link-relations/link-relations.xhtml)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LinkRelation {
    Alternate,
    Collection,
    Conformance,
    Current,
    Data,
    Describedby,
    DerivedFrom,
    Exceptions,
    Execute,
    First,
    Item,
    Items,
    Last,
    License,
    Next,
    Parent,
    Previous,
    ProcessDesc,
    Processes,
    Results,
    Root,
    #[serde(rename = "self")]
    Selfie,
    ServiceDesc,
    ServiceDoc,
    Start,
    Status,
    Tiles,
    Up,
}

impl Default for LinkRelation {
    fn default() -> Self {
        LinkRelation::Selfie
    }
}
