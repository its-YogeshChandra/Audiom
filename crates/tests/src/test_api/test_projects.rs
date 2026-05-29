//all the requests for the projects endpoints
use reqwest;
use serde_json;
use uuid::Uuid;
use super::test_auth::make_token;

const BASE_URL: &str = "http://localhost:8080";

// ══════════════════════════════════════════════
//  Step 1: Create project (owner/admin only)
// ══════════════════════════════════════════════
pub async fn test_create_project(owner_id: &str, workspace_id: &str) {
    println!("CREATE PROJECT");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/workspaces/{}/projects", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Test Project",
            "description": "A test project for integration testing",
            "artwork_url": "https://example.com/artwork.png",
            "rss_slug": "test-project-rss"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[CREATE PROJECT] status: {} | body: {}", status, body);
            println!("  ↳ SAVE THE 'id' FROM ABOVE — this is the project_id\n");
        }
        Err(error) => {
            println!("[CREATE PROJECT] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 2: Create project with minimal fields
// ══════════════════════════════════════════════
pub async fn test_create_project_minimal(owner_id: &str, workspace_id: &str) {
    println!("CREATE PROJECT (minimal — name only)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/workspaces/{}/projects", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Minimal Project"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[CREATE PROJECT MINIMAL] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[CREATE PROJECT MINIMAL] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 3: List all projects in workspace (any member)
// ══════════════════════════════════════════════
pub async fn test_get_projects(user_id_str: &str, workspace_id: &str) {
    println!("GET PROJECTS (list)");

    let user_id = Uuid::parse_str(user_id_str).expect("Invalid user UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/{}/projects", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET PROJECTS] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[GET PROJECTS] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 4: Get single project by ID (any member)
// ══════════════════════════════════════════════
pub async fn test_get_project_by_id(user_id_str: &str, workspace_id: &str, project_id: &str) {
    println!("GET PROJECT BY ID");

    let user_id = Uuid::parse_str(user_id_str).expect("Invalid user UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET PROJECT BY ID] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[GET PROJECT BY ID] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 5: Update project (owner/admin only)
// ══════════════════════════════════════════════
pub async fn test_update_project(owner_id: &str, workspace_id: &str, project_id: &str) {
    println!("UPDATE PROJECT");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Updated Project Name",
            "description": "Updated description for the project"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[UPDATE PROJECT] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[UPDATE PROJECT] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 6: Update project partial (single field)
// ══════════════════════════════════════════════
pub async fn test_update_project_partial(owner_id: &str, workspace_id: &str, project_id: &str) {
    println!("UPDATE PROJECT (partial — rss_slug only)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "rss_slug": "updated-rss-slug"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[UPDATE PROJECT PARTIAL] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[UPDATE PROJECT PARTIAL] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 7: Create project as non-owner/admin (should fail — RBAC)
// ══════════════════════════════════════════════
pub async fn test_create_project_as_viewer(viewer_id: &str, workspace_id: &str) {
    println!("CREATE PROJECT AS VIEWER (expect 403)");

    let user_id = Uuid::parse_str(viewer_id).expect("Invalid viewer UUID");
    let token = make_token(Some(user_id), "viewer@test.com", "Viewer User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/workspaces/{}/projects", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Unauthorized Project",
            "description": "This should not be created"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[CREATE AS VIEWER] status: {} | body: {}", status, body);
            println!("  ↳ Expected: 403 Forbidden\n");
        }
        Err(error) => {
            println!("[CREATE AS VIEWER] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 8: Update project as non-owner/admin (should fail — RBAC)
// ══════════════════════════════════════════════
pub async fn test_update_project_as_viewer(viewer_id: &str, workspace_id: &str, project_id: &str) {
    println!("UPDATE PROJECT AS VIEWER (expect 403)");

    let user_id = Uuid::parse_str(viewer_id).expect("Invalid viewer UUID");
    let token = make_token(Some(user_id), "viewer@test.com", "Viewer User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Hacked Project Name"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[UPDATE AS VIEWER] status: {} | body: {}", status, body);
            println!("  ↳ Expected: 403 Forbidden\n");
        }
        Err(error) => {
            println!("[UPDATE AS VIEWER] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 9: Delete project as non-owner/admin (should fail — RBAC)
// ══════════════════════════════════════════════
pub async fn test_delete_project_as_viewer(viewer_id: &str, workspace_id: &str, project_id: &str) {
    println!("DELETE PROJECT AS VIEWER (expect 403)");

    let user_id = Uuid::parse_str(viewer_id).expect("Invalid viewer UUID");
    let token = make_token(Some(user_id), "viewer@test.com", "Viewer User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[DELETE AS VIEWER] status: {} | body: {}", status, body);
            println!("  ↳ Expected: 403 Forbidden\n");
        }
        Err(error) => {
            println!("[DELETE AS VIEWER] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 10: Get project with wrong workspace (should fail — 404)
// ══════════════════════════════════════════════
pub async fn test_get_project_wrong_workspace(owner_id: &str, fake_workspace_id: &str, project_id: &str) {
    println!("GET PROJECT WITH WRONG WORKSPACE (expect 404)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/{}/projects/{}", BASE_URL, fake_workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET WRONG WORKSPACE] status: {} | body: {}", status, body);
            println!("  ↳ Expected: 404 or 403\n");
        }
        Err(error) => {
            println!("[GET WRONG WORKSPACE] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 11: Delete project (owner/admin only) — run last!
// ══════════════════════════════════════════════
pub async fn test_delete_project(owner_id: &str, workspace_id: &str, project_id: &str) {
    println!("DELETE PROJECT");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[DELETE PROJECT] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[DELETE PROJECT] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 12: Verify delete — get deleted project (should fail — 404)
// ══════════════════════════════════════════════
pub async fn test_get_deleted_project(owner_id: &str, workspace_id: &str, project_id: &str) {
    println!("GET DELETED PROJECT (expect 404)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/{}/projects/{}", BASE_URL, workspace_id, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET DELETED PROJECT] status: {} | body: {}", status, body);
            println!("  ↳ Expected: 404 Not Found\n");
        }
        Err(error) => {
            println!("[GET DELETED PROJECT] error: {:#?}", error);
        }
    }
}