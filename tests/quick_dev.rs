use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub id: Uuid,
    pub email: String,
}

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:3000")?;
    http_client.do_get("/health").await?.print().await?;

    http_client
        .do_post("/users", json!({"email": "email@email.com"}))
        .await?
        .print()
        .await?;

    let res = http_client
        .do_post("/users", json!({ "email": "email@email.com" }))
        .await?;

    let user: User = res.json_body_as()?;
    let user_id: Uuid = user.id;
    let user_endpoint = format!("/users/{}", user_id);

    http_client.do_get(&user_endpoint).await?.print().await?;

    Ok(())
}
