use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    version: i8,
    pub features: Vec<Toggle>,
    pub segments: Option<Vec<Segment>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Toggle {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub strategies: Vec<Strategy>,
    #[serde(default = "empty_vec")]
    pub variants: Vec<VariantDef>,
}

fn empty_vec() -> Vec<VariantDef> {
    vec![]
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Variant {
    pub name: String,
    pub payload: Option<VariantPayload>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariantDef {
    pub name: String,
    pub weight: u8,
    pub payload: VariantPayload,
    pub overrides: Option<Vec<Override>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Override {
    pub context_name: String,
    pub values: Vec<String>,
}

impl Default for Variant {
    fn default() -> Self {
        Variant {
            name: "disabled".to_string(),
            payload: None,
            enabled: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VariantPayload {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub variant_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Strategy {
    pub name: String,
    pub parameters: Option<HashMap<String, String>>,
    constraints: Option<Vec<Constraint>>,
    segments: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Operator {
    In,
    NotIn,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Constraint {
    context_name: String,
    values: Vec<String>,
    value: String,
    operator: Operator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Segment {
    id: i32,
    constraints: Vec<Constraint>,
}
