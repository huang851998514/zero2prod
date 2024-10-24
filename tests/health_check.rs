#[tokio::test]
async fn health_check_works () {
    //准备
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
    .get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("失败的请求");

    assert!(response.status().is_success());
    assert_eq!(Some(0),response.content_length());
}


fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:0").expect("无法绑定端口");
    let _ = tokio::spawn(server);
}