use std::net::TcpListener;
use reqwest;

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();
    let endpoint = &format!("{}/health_check", &address);
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn greet_test() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let resp = client
        .get(address)
        .send()
        .await
        .expect("Failed to execute greet");
    assert!(resp.status().is_success());
    let body = resp.text().await.expect("Failed to get text");
    println!("{:?}", body);
    assert_eq!(body, "Hello World");
}

#[tokio::test]
async fn greet_name_test() {
    let address = spawn_app();
    let endpoint = &format!("{}/some_name", &address);
    let client = reqwest::Client::new();
    let resp = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute greet");
    assert!(resp.status().is_success());
    let body = resp.text().await.expect("Failed to get text");
    println!("{:?}", body);
    assert_eq!(body, "Hello some_name");
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_resturns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let endpoint = &format!("{}/subscriptions", &address);
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let client = reqwest::Client::new();
    let resp = client
        .post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to sned to subscriptions endpoint");
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn subscribe_resturns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let endpoint = &format!("{}/subscriptions", &address);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let resp = client
            .post(endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to sned to subscriptions endpoint");
        assert_eq!(400,
            resp.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}