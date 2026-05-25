//all the request for the workspaces endpoints
use reqwest;
use serde_json;

async fn test_create_workspace() -> Result<(),Box<dyn std::error::Error>>{
    
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/workspaces")
        .header("Authorization", "Bearer ")
        .json(&serde_json::json!({
            "name": "Test Workspace",
            "slug": "test-workspace",
            "plan": "free"
        }))
        .send()
        .await?;

    Ok(())

}