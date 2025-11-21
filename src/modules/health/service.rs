use serde_json::{json, Value};

pub async fn index() -> Value {
  json!({ "status": "ok" })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_health_index_returns_ok() {
    let result = index().await;
    assert_eq!(result, json!({ "status": "ok" }));
  }

  #[tokio::test]
  async fn test_health_index_has_status_field() {
    let result = index().await;
    assert!(result.get("status").is_some());
    assert_eq!(result.get("status").unwrap(), "ok");
  }
}
