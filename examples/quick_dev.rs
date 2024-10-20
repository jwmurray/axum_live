use anyhow::Ok;
#[allow(unused)]
use anyhow::Result;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    sleep(Duration::from_secs(1)).await;

    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Here is a quick get which prints the response and consumes the response:
    hc.do_get("/hello?name=Jennifer").await?.print().await?; // Print the response
    hc.do_get("/hello/Mike").await?.print().await?; // Print the response

    // Here we save the response and print and assert its value on separate lines, taking 3 lines instead of 1
    let response = hc.do_get("/hello?name=Jennifer").await?;
    response.print().await?;
    assert_eq!(response.text_body()?, "Hello, <strong>Jennifer</strong>"); // Check the response body

    // Again we save the response and print and assert its value on separate lines, taking 3 lines instead of 1
    let response = hc.do_get("/hello/Mike").await?;
    response.print().await?;
    assert_eq!(response.text_body()?, "Hello, <strong>Mike</strong>"); // Check the response body

    hc.do_get("/src/main.rs").await?.print().await?; // Print the response

    // Invalid password, fails
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcomedds" 
        }),
    );
    req_login.await?.print().await?;

    // // Valid password, succedss
    // let req_login = hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "demo1",
    //         "pwd": "welcome"
    //     }),
    // );
    // req_login.await?.print().await?; // Print the response 

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "My first ticket"
        }),
    );
    req_create_ticket.await?.print().await?; // Print the response

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "My second ticket"
        }),
    );
    req_create_ticket.await?.print().await?; // Print the response

    // hc.do_delete("/api/tickets/1").await?.print().await?; // Print the response")

    hc.do_get("/api/tickets").await?.print().await?; // Print the response

    Ok(())
}
