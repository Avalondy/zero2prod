use std::net::TcpListener;

#[tokio::test]
async fn greet_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&address)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let respond_text = response.text().await.unwrap();
    assert_eq!(respond_text, "Hello World!");

    let response = client
        .get(address.clone() + "/test")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let respond_text = response.text().await.unwrap();
    assert_eq!(respond_text, "Hello test!");

    println!("{address}");
}

/// Launch the app in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    // Return the application address
    format!("http://127.0.0.1:{port}")
}
