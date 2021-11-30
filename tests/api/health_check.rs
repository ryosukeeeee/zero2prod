use crate::helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let address = app.address;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    //
    // use `cargo add reqwest --dev --vers 0.11` to add
    // it under `[dev-dependencies]` in Cargo.toml
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
