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