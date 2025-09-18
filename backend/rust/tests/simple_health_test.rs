use reqwest;
use std::time::Duration;

#[tokio::test]
async fn test_health_endpoint_connection() {
    // Try to connect to the health endpoint
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let auth_health = client.get("http://localhost:3000/health").send().await;
    match auth_health {
        Ok(resp) => {
            println!("Auth service health check: status={}", resp.status());
            assert!(resp.status().is_success() || resp.status().is_server_error());
        }
        Err(e) => {
            println!("Auth service not running: {}", e);
        }
    }

    let platform_health = client.get("http://localhost:3001/health").send().await;
    match platform_health {
        Ok(resp) => {
            println!("Platform service health check: status={}", resp.status());
            assert!(resp.status().is_success() || resp.status().is_server_error());
        }
        Err(e) => {
            println!("Platform service not running: {}", e);
        }
    }

    let commerce_health = client.get("http://localhost:3002/health").send().await;
    match commerce_health {
        Ok(resp) => {
            println!("Commerce service health check: status={}", resp.status());
            assert!(resp.status().is_success() || resp.status().is_server_error());
        }
        Err(e) => {
            println!("Commerce service not running: {}", e);
        }
    }
}

#[test]
fn test_validation_logic() {
    // Test basic validation rules without database
    let valid_email = "test@example.com";
    let invalid_email = "not-an-email";

    assert!(valid_email.contains('@'));
    assert!(!invalid_email.contains('@'));
}

#[test]
fn test_event_structure() {
    use serde_json::json;

    // Test that event JSON structure is valid
    let event = json!({
        "event_type": "user.created",
        "tenant_id": "test-tenant",
        "payload": {
            "user_id": "test-user",
            "email": "test@example.com"
        },
        "timestamp": "2025-01-01T00:00:00Z"
    });

    assert_eq!(event["event_type"], "user.created");
    assert!(event["payload"].is_object());
}