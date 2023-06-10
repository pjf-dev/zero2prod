use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get( &format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {

    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    let req_body = "name=le%20guin&email=ursula_le_guin%40@gmail.com";
    let res = client
        .post(&format!("{}/subscribe", &app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(req_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(res.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {

    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email")
    ];

    for (body, err_msg) in test_cases {
        let res = client
            .post(&format!("{}/subscribe", &app_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(res.status().as_u16(), 400,
                   "API did not respond with status 400 when payload was {}.", err_msg);
    }

}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
