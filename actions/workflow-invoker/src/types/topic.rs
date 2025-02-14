use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(skip_serializing, rename(deserialize = "_id"))]
    pub id: String,
    #[serde(skip_serializing, rename(deserialize = "_rev"))]
    pub rev: String,
    pub data: Vec<Value>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row<T> {
    rows: Vec<View<T>>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View<T> {
    doc: T,
}
