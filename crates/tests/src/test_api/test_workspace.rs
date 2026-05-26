//all the request for the workspaces endpoints
use reqwest;
use serde_json;
use uuid::Uuid;
use super::test_auth::make_token;

// ── Test user definitions ──
// 5 test users for workspace testing
// After signup, their DB-assigned UUIDs will be printed — you'll paste them below

struct TestUser {
    email: &'static str,
    name: &'static str,
    password: &'static str,
}

const TEST_USERS: [TestUser; 5] = [
    TestUser { email: "owner@test.com",   name: "Owner User",   password: "pass123" },
    TestUser { email: "admin@test.com",   name: "Admin User",   password: "pass123" },
    TestUser { email: "editor@test.com",  name: "Editor User",  password: "pass123" },
    TestUser { email: "viewer@test.com",  name: "Viewer User",  password: "pass123" },
    TestUser { email: "guest@test.com",   name: "Guest User",   password: "pass123" },
];

const BASE_URL: &str = "http://localhost:8080";

// ══════════════════════════════════════════════
//  Step 1: Sign up all 5 test users
// ══════════════════════════════════════════════
pub async fn test_signup_users() {
    println!("SIGNUP USERS (5 users)");

    let client = reqwest::Client::new();

    for (i, user) in TEST_USERS.iter().enumerate() {
        let token = make_token(None, user.email, user.name, user.password);

        let response = client
            .post(format!("{}/signup", BASE_URL))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                println!("[SIGNUP USER {}] {} | status: {} | body: {}", i + 1, user.email, status, body);
                println!("  ↳ SAVE THE 'id' FROM ABOVE — you'll need it for workspace tests\n");
            }
            Err(error) => {
                println!("[SIGNUP USER {}] {} | error: {:#?}\n", i + 1, user.email, error);
            }
        }
    }
}

// ══════════════════════════════════════════════
//  Step 2: Login all 5 users (verify they exist)
// ══════════════════════════════════════════════
pub async fn test_login_users() {
    println!("LOGIN USERS (5 users)");

    let client = reqwest::Client::new();

    for (i, user) in TEST_USERS.iter().enumerate() {
        let token = make_token(None, user.email, user.name, user.password);

        let response = client
            .post(format!("{}/login", BASE_URL))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                println!("[LOGIN USER {}] {} | status: {} | body: {}", i + 1, user.email, status, body);
            }
            Err(error) => {
                println!("[LOGIN USER {}] {} | error: {:#?}", i + 1, user.email, error);
            }
        }
    }
}

// ══════════════════════════════════════════════
//  Step 3: Create workspace (as user1/owner)
// ══════════════════════════════════════════════
pub async fn test_create_workspace(owner_id: &str) {
    println!("CREATE WORKSPACE");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/workspaces", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Test Workspace",
            "slug": "test-workspace",
            "plan": "free"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[CREATE WORKSPACE] status: {} | body: {}", status, body);
            println!("  ↳ SAVE THE 'id' FROM ABOVE — this is the workspace_id\n");
        }
        Err(error) => {
            println!("[CREATE WORKSPACE] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 4: Get workspaces (list for owner)
// ══════════════════════════════════════════════
pub async fn test_get_workspaces(owner_id: &str) {
    println!("GET WORKSPACES (list)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET WORKSPACES] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[GET WORKSPACES] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 5: Get workspace by slug
// ══════════════════════════════════════════════
pub async fn test_get_workspace_by_slug(owner_id: &str) {
    println!("GET WORKSPACE BY SLUG");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/test-workspace", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[GET BY SLUG] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[GET BY SLUG] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 6: Update workspace (owner changes name)
// ══════════════════════════════════════════════
pub async fn test_update_workspace(owner_id: &str, workspace_id: &str) {
    println!("UPDATE WORKSPACE");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let ws_id = workspace_id;
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}/workspaces/{}", BASE_URL, ws_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "Updated Workspace Name"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[UPDATE WORKSPACE] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[UPDATE WORKSPACE] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 7: List members (should be just owner)
// ══════════════════════════════════════════════
pub async fn test_list_members(owner_id: &str, workspace_id: &str) {
    println!("LIST MEMBERS");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/workspaces/{}/members", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[LIST MEMBERS] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[LIST MEMBERS] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 8: Add members (owner adds user2=admin, user3=editor, user4=viewer, user5=guest)
// ══════════════════════════════════════════════
pub async fn test_add_members(owner_id: &str, workspace_id: &str) {
    println!("ADD MEMBERS (4 users)");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let members_to_add = [
        ("admin@test.com",  "admin"),
        ("editor@test.com", "editor"),
        ("viewer@test.com", "viewer"),
        ("guest@test.com",  "guest"),
    ];

    let client = reqwest::Client::new();

    for (email, role) in members_to_add.iter() {
        let response = client
            .post(format!("{}/workspaces/{}/members", BASE_URL, workspace_id))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": email,
                "role": role
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                println!("[ADD MEMBER] {} as {} | status: {} | body: {}", email, role, status, body);
            }
            Err(error) => {
                println!("[ADD MEMBER] {} as {} | error: {:#?}", email, role, error);
            }
        }
    }
}

// ══════════════════════════════════════════════
//  Step 9: Change role (owner changes user3 from editor → admin)
// ══════════════════════════════════════════════
pub async fn test_change_role(owner_id: &str, workspace_id: &str, target_user_id: &str) {
    println!("CHANGE MEMBER ROLE");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}/workspaces/{}/members/{}/role", BASE_URL, workspace_id, target_user_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "role": "admin"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[CHANGE ROLE] target: {} → admin | status: {} | body: {}", target_user_id, status, body);
        }
        Err(error) => {
            println!("[CHANGE ROLE] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 10: Remove member (owner removes user5/guest)
// ══════════════════════════════════════════════
pub async fn test_remove_member(owner_id: &str, workspace_id: &str, target_user_id: &str) {
    println!("REMOVE MEMBER");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{}/workspaces/{}/members/{}", BASE_URL, workspace_id, target_user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[REMOVE MEMBER] target: {} | status: {} | body: {}", target_user_id, status, body);
        }
        Err(error) => {
            println!("[REMOVE MEMBER] error: {:#?}", error);
        }
    }
}

// ══════════════════════════════════════════════
//  Step 11: Delete workspace (owner only)
// ══════════════════════════════════════════════
pub async fn test_delete_workspace(owner_id: &str, workspace_id: &str) {
    println!("DELETE WORKSPACE");

    let user_id = Uuid::parse_str(owner_id).expect("Invalid owner UUID");
    let token = make_token(Some(user_id), "owner@test.com", "Owner User", "pass123");

    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{}/workspaces/{}", BASE_URL, workspace_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[DELETE WORKSPACE] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[DELETE WORKSPACE] error: {:#?}", error);
        }
    }
}