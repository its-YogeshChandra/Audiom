//all the request for the workspaces endpoints
use reqwest;
use serde_json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateWorkspaceBody {
    pub name: String,
    pub slug: String,
    pub plan: Option<String>,
}

//create a static variable for the workspace id 
static mut WORKSPACE_ID: Option<String> = None;

async fn test_create_workspace() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let body = CreateWorkspaceBody{
        name:"Test Workspace".to_string(),
        slug:"test-workspace".to_string(),
        plan:Some("free".to_string()),
    };

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

async fn test_get_workspaces() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/workspaces")
        .header("Authorization", "Bearer ")
        .send()
        .await?;

    Ok(())
}

async fn test_get_workspace_by_slug() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/workspaces/test-workspace")
        .header("Authorization", "Bearer ")
        .send()
        .await?;
    
    Ok(())
}

async fn test_update_workspace() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let body = UpdateWorkspaceBody{
        name:"Test Workspace".to_string(),
        slug:"test-workspace".to_string(),
        plan:Some("free".to_string()),
    };

    let client = reqwest::Client::new();
    let response = client
        .put("http://localhost:8080/workspaces/test-workspace")
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

async fn test_delete_workspace() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let client = reqwest::Client::new();
    let response = client
        .delete("http://localhost:8080/workspaces/test-workspace")
        .header("Authorization", "Bearer ")
        .send()
        .await?;
    
    Ok(())
}

async fn test_list_workspace_members() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/workspaces/test-workspace/members")
        .header("Authorization", "Bearer ")
        .send()
        .await?;
    
    Ok(())
}

async fn test_add_member() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let body = AddMemberBody{
        email:"test-workspace".to_string(),
        role:"free".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/workspaces/test-workspace/members")
        .header("Authorization", "Bearer ")
        .json(&serde_json::json!({
            "email": "test-workspace",
            "role": "free"
        }))
        .send()
        .await?;
    
    Ok(())
}

async fn test_remove_member() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let client = reqwest::Client::new();
    let response = client
        .delete("http://localhost:8080/workspaces/test-workspace/members")
        .header("Authorization", "Bearer ")
        .send()
        .await?;
    
    Ok(())
}

async fn test_change_member_role() -> Result<(),Box<dyn std::error::Error>>{
    
    let jwt_token = "".to_string(); 

    let body = ChangeRoleBody{
        role:"free".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .put("http://localhost:8080/workspaces/test-workspace/members")
        .header("Authorization", "Bearer ")
        .json(&serde_json::json!({
            "role": "free"
        }))
        .send()
        .await?;
    
    Ok(())
}