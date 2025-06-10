use serde_json::{json, Value};

pub async fn index() -> Value {
  json!({ "status": "ok" })
}
