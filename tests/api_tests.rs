use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use fusegu::{
    config::Config,
    server::create_app,
};

/// Helper function to create a test app
async fn create_test_app() -> Router {
    let config = Config::default();
    create_app(config).await.unwrap()
}

/// Helper function to make a request and get the response
async fn make_request(app: &Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    
    // Try to parse as JSON, if it fails return the raw text for debugging
    let json: Value = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(_) => {
            let text = String::from_utf8_lossy(&body);
            println!("Raw response body: {}", text);
            json!({})
        }
    };
    (status, json)
}

// TRANSACTION ENDPOINT TESTS

#[tokio::test]
async fn test_post_transactions_success() {
    let app = create_test_app().await;
    
    let transaction_request = json!({
        "device": {
            "ip_address": "198.51.100.1",
            "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        },
        "event": {
            "event_type": "purchase",
            "transaction_id": "txn_123456789",
            "time": "2025-06-13T10:30:00Z"
        },
        "user_id": "550e8400-e29b-41d4-a716-446655440001",
        "order": {
            "amount": 199.99,
            "currency": "USD"
        },
        "email": {
            "address": "customer@example.com"
        },
        "billing": {
            "country": "US",
            "postal": "10001"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(transaction_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(response.get("id").is_some());
    assert!(response.get("risk_score").is_some());
    assert!(response.get("risk_level").is_some());
    assert!(response.get("disposition").is_some());
    assert!(response.get("created_at").is_some());
    assert!(response.get("_links").is_some());
    
    // Validate risk_score - it's returned as a string from Decimal
    let risk_score_str = response["risk_score"].as_str().unwrap();
    let risk_score: f64 = risk_score_str.parse().unwrap();
    assert!(risk_score >= 0.01 && risk_score <= 99.99);
    
    // Validate enums are correct
    let risk_level = response["risk_level"].as_str().unwrap();
    assert!(["low", "medium", "high", "very_high"].contains(&risk_level));
    
    let disposition = response["disposition"].as_str().unwrap();
    assert!(["accept", "reject", "review", "test"].contains(&disposition));
}

#[tokio::test]
async fn test_post_transactions_validation_error() {
    let app = create_test_app().await;
    
    // Missing required device field
    let invalid_request = json!({
        "event": {
            "event_type": "purchase"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(invalid_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.get("error").is_some());
    assert!(response.get("message").is_some());
}

#[tokio::test]
async fn test_get_transaction_by_id_success() {
    let app = create_test_app().await;
    
    let transaction_id = "550e8400-e29b-41d4-a716-446655440000";
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/v1/transactions/{}", transaction_id))
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(response["id"], transaction_id);
    assert!(response.get("risk_score").is_some());
    assert!(response.get("risk_level").is_some());
    assert!(response.get("disposition").is_some());
    assert!(response.get("created_at").is_some());
    assert!(response.get("_links").is_some());
}

#[tokio::test]
async fn test_list_transactions_default() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/transactions")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert!(response.get("transactions").is_some());
    assert!(response.get("pagination").is_some());
    assert!(response.get("_links").is_some());
    
    let transactions = response["transactions"].as_array().unwrap();
    assert!(transactions.len() <= 20); // Default limit
    
    let pagination = &response["pagination"];
    assert!(pagination.get("limit").is_some());
    assert!(pagination.get("offset").is_some());
    assert!(pagination.get("total").is_some());
    assert!(pagination.get("has_more").is_some());
}

// USER ENDPOINT TESTS

#[tokio::test]
async fn test_post_users_success() {
    let app = create_test_app().await;
    
    let user_request = json!({
        "external_user_id": "user_12345",
        "user_hash": "5d41402abc4b2a76b9719d911017c592",
        "metadata": {
            "registration_date": "2025-06-01",
            "user_type": "premium"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/users")
        .header("content-type", "application/json")
        .body(Body::from(user_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(response.get("id").is_some());
    assert_eq!(response["external_user_id"], "user_12345");
    assert_eq!(response["user_hash"], "5d41402abc4b2a76b9719d911017c592");
    assert!(response.get("risk_score").is_some());
    assert!(response.get("risk_level").is_some());
    assert!(response.get("total_transactions").is_some());
    assert!(response.get("created_at").is_some());
    assert!(response.get("_links").is_some());
}

#[tokio::test]
async fn test_get_user_by_id_success() {
    let app = create_test_app().await;
    
    let user_id = "550e8400-e29b-41d4-a716-446655440000";
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/v1/users/{}", user_id))
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(response["id"], user_id);
    assert!(response.get("risk_score").is_some());
    assert!(response.get("risk_level").is_some());
    assert!(response.get("total_transactions").is_some());
    assert!(response.get("successful_transactions").is_some());
    assert!(response.get("failed_transactions").is_some());
    assert!(response.get("chargeback_count").is_some());
    assert!(response.get("is_verified").is_some());
    assert!(response.get("is_flagged").is_some());
    assert!(response.get("created_at").is_some());
    assert!(response.get("_links").is_some());
}

#[tokio::test]
async fn test_list_users_default() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/users")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert!(response.get("users").is_some());
    assert!(response.get("pagination").is_some());
    assert!(response.get("_links").is_some());
    
    let users = response["users"].as_array().unwrap();
    assert!(users.len() <= 20); // Default limit
    
    let pagination = &response["pagination"];
    assert!(pagination.get("limit").is_some());
    assert!(pagination.get("offset").is_some());
    assert!(pagination.get("total").is_some());
    assert!(pagination.get("has_more").is_some());
}

// ACCOUNT ENDPOINT TESTS

#[tokio::test]
async fn test_get_account_success() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/account")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert!(response.get("account_id").is_some());
    assert!(response.get("subscription_tier").is_some());
    assert!(response.get("funds_remaining").is_some());
    assert!(response.get("monthly_quota").is_some());
    assert!(response.get("queries_used_this_month").is_some());
    assert!(response.get("queries_remaining_this_month").is_some());
    assert!(response.get("billing_cycle_start").is_some());
    assert!(response.get("billing_cycle_end").is_some());
    
    // Validate subscription tier enum
    let subscription_tier = response["subscription_tier"].as_str().unwrap();
    assert!(["free", "pro", "enterprise"].contains(&subscription_tier));
    
    // Validate numeric fields are non-negative
    let funds_remaining = response["funds_remaining"].as_f64().unwrap();
    assert!(funds_remaining >= 0.0);
    
    let monthly_quota = response["monthly_quota"].as_i64().unwrap();
    assert!(monthly_quota >= 0);
    
    let queries_used = response["queries_used_this_month"].as_i64().unwrap();
    assert!(queries_used >= 0);
    
    let queries_remaining = response["queries_remaining_this_month"].as_i64().unwrap();
    assert!(queries_remaining >= 0);
}

// ADDITIONAL TEST CASES

#[tokio::test]
async fn test_list_transactions_with_filters() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/transactions?limit=10&offset=5&risk_level=high&disposition=reject&sort=-risk_score")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    
    let transactions = response["transactions"].as_array().unwrap();
    assert!(transactions.len() <= 10);
    
    let pagination = &response["pagination"];
    assert_eq!(pagination["limit"], 10);
    assert_eq!(pagination["offset"], 5);
}

#[tokio::test]
async fn test_list_users_with_filters() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/users?limit=15&offset=10&risk_level=medium&is_flagged=false&is_verified=true&sort=-risk_score")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    
    let users = response["users"].as_array().unwrap();
    assert!(users.len() <= 15);
    
    let pagination = &response["pagination"];
    assert_eq!(pagination["limit"], 15);
    assert_eq!(pagination["offset"], 10);
}

#[tokio::test]
async fn test_patch_user_success() {
    let app = create_test_app().await;
    
    let user_id = "550e8400-e29b-41d4-a716-446655440000";
    let update_request = json!({
        "is_verified": true,
        "is_flagged": false,
        "flags": ["verified_phone", "verified_email"],
        "metadata": {
            "verification_date": "2025-06-13",
            "verification_method": "phone"
        }
    });

    let request = Request::builder()
        .method("PATCH")
        .uri(&format!("/v1/users/{}", user_id))
        .header("content-type", "application/json")
        .body(Body::from(update_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(response["id"], user_id);
    assert_eq!(response["is_verified"], true);
    assert_eq!(response["is_flagged"], false);
    assert!(response.get("updated_at").is_some());
}

#[tokio::test]
async fn test_invalid_uuid_error() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/v1/transactions/invalid-uuid")
        .body(Body::empty())
        .unwrap();

    let (status, _) = make_request(&app, request).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_json_error() {
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from("invalid json"))
        .unwrap();

    let (status, _) = make_request(&app, request).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_large_transaction_amount() {
    let app = create_test_app().await;
    
    let transaction_request = json!({
        "device": {
            "ip_address": "198.51.100.1",
            "user_agent": "Mozilla/5.0 (compatible; Bot/1.0)"
        },
        "event": {
            "event_type": "purchase",
            "transaction_id": "txn_large_amount",
            "time": "2025-06-13T10:30:00Z"
        },
        "order": {
            "amount": 99999.99,
            "currency": "USD"
        },
        "email": {
            "address": "highvalue@example.com"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(transaction_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    
    // Large amounts might trigger higher risk scores - risk_score is a string
    let risk_score_str = response["risk_score"].as_str().unwrap();
    let risk_score: f64 = risk_score_str.parse().unwrap();
    assert!(risk_score >= 0.01 && risk_score <= 99.99);
}

#[tokio::test]
async fn test_minimal_user_creation() {
    let app = create_test_app().await;
    
    let user_request = json!({
        "user_hash": "098f6bcd4621d373cade4e832627b4f6"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/users")
        .header("content-type", "application/json")
        .body(Body::from(user_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(response.get("id").is_some());
    assert_eq!(response["user_hash"], "098f6bcd4621d373cade4e832627b4f6");
}

#[tokio::test]
async fn test_pagination_limits() {
    let app = create_test_app().await;
    
    // Test maximum pagination limit
    let request = Request::builder()
        .method("GET")
        .uri("/v1/transactions?limit=1000")
        .body(Body::empty())
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::OK);
    
    let pagination = &response["pagination"];
    // Should cap at maximum allowed limit (likely 100)
    let limit = pagination["limit"].as_i64().unwrap();
    assert!(limit <= 100);
}

#[tokio::test]
async fn test_risk_level_consistency() {
    let app = create_test_app().await;
    
    let transaction_request = json!({
        "device": {
            "ip_address": "198.51.100.1",
            "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        },
        "event": {
            "event_type": "purchase",
            "transaction_id": "txn_consistency_test",
            "time": "2025-06-13T10:30:00Z"
        },
        "order": {
            "amount": 50.00,
            "currency": "USD"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(transaction_request.to_string()))
        .unwrap();

    let (status, response) = make_request(&app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    
    let risk_score_str = response["risk_score"].as_str().unwrap();
    let risk_score: f64 = risk_score_str.parse().unwrap();
    let risk_level = response["risk_level"].as_str().unwrap();
    let disposition = response["disposition"].as_str().unwrap();
    
    // Verify risk level is consistent with risk score
    match risk_level {
        "low" => assert!(risk_score < 30.0),
        "medium" => assert!(risk_score >= 10.0 && risk_score < 70.0),
        "high" | "very_high" => assert!(risk_score >= 30.0),
        _ => panic!("Invalid risk level: {}", risk_level),
    }
    
    // Verify disposition matches risk assessment
    match disposition {
        "accept" => assert!(["low", "medium"].contains(&risk_level)),
        "review" => assert!(["medium", "high"].contains(&risk_level)),
        "reject" => assert!(["high", "very_high"].contains(&risk_level)),
        "test" => {} // Test disposition can be any risk level
        _ => panic!("Invalid disposition: {}", disposition),
    }
}

// HEALTH AND ROOT ENDPOINT TESTS

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_root_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
} 