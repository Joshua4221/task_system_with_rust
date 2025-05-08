#[allow(unused)]
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8010")?;

    let req_task = hc.do_post("/api/task",
    json!({"title": "work on system".to_string(),
         "status": "online".to_string(),
         "description": "Join our backend team to build scalable microservices in Rust. You'll work with async architectures, contribute to open-source, and shape the core infrastructure of our growing platform.".to_string(),})
    );

    req_task.await?.print().await?;

    hc.do_get("/api/task").await?.print().await?;

    hc.do_get("/api/task/0").await?.print().await?;

    let req_update_task = hc.do_patch("/api/task/0",
    json!({"title": "perfect system".to_string(),
         "status": "online".to_string(),
         "description": "Join our backend team to build scalable microservices in Rust. You'll work with async architectures, contribute to open-source, and shape the core infrastructure of our growing platform.".to_string(),})
    );

    req_update_task.await?.print().await?;

    hc.do_delete("/api/task/0").await?.print().await?;

    Ok(())
}
