use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Example {
    pub metadata: Option<serde_json::Value>,
    pub ground_truth: Option<String>,
    pub text: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelledExample {
    pub example: Example,
    pub label: Option<String>
}
