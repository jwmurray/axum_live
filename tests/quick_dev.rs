use anyhow::Ok;
#[allow(unused)]
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    sleep(Duration::from_secs(1)).await;

    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Jenny").await?.print().await?;

    Ok(())
}
