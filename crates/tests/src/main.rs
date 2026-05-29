mod test_api;
use tokio;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <test_name> [args...]");
        println!("");
        println!("Available tests (run in this order):");
        println!("");
        println!("── Auth ──");
        println!("  test_signup                                           — Original signup test (marco)");
        println!("  test_login                                            — Original login test (marco)");
        println!("");
        println!("── Workspace ──");
        println!("  signup_users                                          — Sign up 5 test users");
        println!("  login_users                                           — Login all 5 users");
        println!("  create_workspace      <owner_id>                      — Create a workspace");
        println!("  get_workspaces        <owner_id>                      — List owner's workspaces");
        println!("  get_workspace_by_slug <owner_id>                      — Get workspace by slug");
        println!("  update_workspace      <owner_id> <workspace_id>       — Update workspace name");
        println!("  list_members          <owner_id> <workspace_id>       — List workspace members");
        println!("  add_members           <owner_id> <workspace_id>       — Add 4 users as members");
        println!("  change_role           <owner_id> <workspace_id> <target_user_id> — Change member role");
        println!("  remove_member         <owner_id> <workspace_id> <target_user_id> — Remove a member");
        println!("  delete_workspace      <owner_id> <workspace_id>       — Delete workspace");
        println!("");
        println!("── Projects ──");
        println!("  create_project             <owner_id> <workspace_id>                      — Create project (full fields)");
        println!("  create_project_minimal     <owner_id> <workspace_id>                      — Create project (name only)");
        println!("  get_projects               <user_id>  <workspace_id>                      — List all projects in workspace");
        println!("  get_project_by_id          <user_id>  <workspace_id> <project_id>         — Get single project");
        println!("  update_project             <owner_id> <workspace_id> <project_id>         — Update project (name + desc)");
        println!("  update_project_partial     <owner_id> <workspace_id> <project_id>         — Update project (rss_slug only)");
        println!("  create_project_as_viewer   <viewer_id> <workspace_id>                     — RBAC: viewer creates (expect 403)");
        println!("  update_project_as_viewer   <viewer_id> <workspace_id> <project_id>        — RBAC: viewer updates (expect 403)");
        println!("  delete_project_as_viewer   <viewer_id> <workspace_id> <project_id>        — RBAC: viewer deletes (expect 403)");
        println!("  get_project_wrong_ws       <owner_id>  <fake_workspace_id> <project_id>   — Cross-workspace (expect 404)");
        println!("  delete_project             <owner_id> <workspace_id> <project_id>         — Delete project");
        println!("  get_deleted_project        <owner_id> <workspace_id> <project_id>         — Verify delete (expect 404)");
        return;
    }

    let test_name = args[1].as_str();

    match test_name {
        // ── Original auth tests ──
        "test_signup" => {
            test_api::test_signup().await;
        }
        "test_login" => {
            test_api::test_login().await;
        }

        // ── Workspace test sequence ──
        "signup_users" => {
            test_api::test_signup_users().await;
        }
        "login_users" => {
            test_api::test_login_users().await;
        }
        "create_workspace" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- create_workspace <owner_id>");
            test_api::test_create_workspace(owner_id).await;
        }
        "get_workspaces" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- get_workspaces <owner_id>");
            test_api::test_get_workspaces(owner_id).await;
        }
        "get_workspace_by_slug" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- get_workspace_by_slug <owner_id>");
            test_api::test_get_workspace_by_slug(owner_id).await;
        }
        "update_workspace" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- update_workspace <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- update_workspace <owner_id> <workspace_id>");
            test_api::test_update_workspace(owner_id, workspace_id).await;
        }
        "list_members" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- list_members <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- list_members <owner_id> <workspace_id>");
            test_api::test_list_members(owner_id, workspace_id).await;
        }
        "add_members" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- add_members <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- add_members <owner_id> <workspace_id>");
            test_api::test_add_members(owner_id, workspace_id).await;
        }
        "change_role" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- change_role <owner_id> <workspace_id> <target_user_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- change_role <owner_id> <workspace_id> <target_user_id>");
            let target_user_id = args.get(4).expect("Usage: cargo run -- change_role <owner_id> <workspace_id> <target_user_id>");
            test_api::test_change_role(owner_id, workspace_id, target_user_id).await;
        }
        "remove_member" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- remove_member <owner_id> <workspace_id> <target_user_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- remove_member <owner_id> <workspace_id> <target_user_id>");
            let target_user_id = args.get(4).expect("Usage: cargo run -- remove_member <owner_id> <workspace_id> <target_user_id>");
            test_api::test_remove_member(owner_id, workspace_id, target_user_id).await;
        }
        "delete_workspace" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- delete_workspace <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- delete_workspace <owner_id> <workspace_id>");
            test_api::test_delete_workspace(owner_id, workspace_id).await;
        }

        // ── Project test sequence ──
        "create_project" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- create_project <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- create_project <owner_id> <workspace_id>");
            test_api::test_create_project(owner_id, workspace_id).await;
        }
        "create_project_minimal" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- create_project_minimal <owner_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- create_project_minimal <owner_id> <workspace_id>");
            test_api::test_create_project_minimal(owner_id, workspace_id).await;
        }
        "get_projects" => {
            let user_id = args.get(2).expect("Usage: cargo run -- get_projects <user_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- get_projects <user_id> <workspace_id>");
            test_api::test_get_projects(user_id, workspace_id).await;
        }
        "get_project_by_id" => {
            let user_id = args.get(2).expect("Usage: cargo run -- get_project_by_id <user_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- get_project_by_id <user_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- get_project_by_id <user_id> <workspace_id> <project_id>");
            test_api::test_get_project_by_id(user_id, workspace_id, project_id).await;
        }
        "update_project" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- update_project <owner_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- update_project <owner_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- update_project <owner_id> <workspace_id> <project_id>");
            test_api::test_update_project(owner_id, workspace_id, project_id).await;
        }
        "update_project_partial" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- update_project_partial <owner_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- update_project_partial <owner_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- update_project_partial <owner_id> <workspace_id> <project_id>");
            test_api::test_update_project_partial(owner_id, workspace_id, project_id).await;
        }
        "create_project_as_viewer" => {
            let viewer_id = args.get(2).expect("Usage: cargo run -- create_project_as_viewer <viewer_id> <workspace_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- create_project_as_viewer <viewer_id> <workspace_id>");
            test_api::test_create_project_as_viewer(viewer_id, workspace_id).await;
        }
        "update_project_as_viewer" => {
            let viewer_id = args.get(2).expect("Usage: cargo run -- update_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- update_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- update_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            test_api::test_update_project_as_viewer(viewer_id, workspace_id, project_id).await;
        }
        "delete_project_as_viewer" => {
            let viewer_id = args.get(2).expect("Usage: cargo run -- delete_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- delete_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- delete_project_as_viewer <viewer_id> <workspace_id> <project_id>");
            test_api::test_delete_project_as_viewer(viewer_id, workspace_id, project_id).await;
        }
        "get_project_wrong_ws" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- get_project_wrong_ws <owner_id> <fake_workspace_id> <project_id>");
            let fake_workspace_id = args.get(3).expect("Usage: cargo run -- get_project_wrong_ws <owner_id> <fake_workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- get_project_wrong_ws <owner_id> <fake_workspace_id> <project_id>");
            test_api::test_get_project_wrong_workspace(owner_id, fake_workspace_id, project_id).await;
        }
        "delete_project" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- delete_project <owner_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- delete_project <owner_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- delete_project <owner_id> <workspace_id> <project_id>");
            test_api::test_delete_project(owner_id, workspace_id, project_id).await;
        }
        "get_deleted_project" => {
            let owner_id = args.get(2).expect("Usage: cargo run -- get_deleted_project <owner_id> <workspace_id> <project_id>");
            let workspace_id = args.get(3).expect("Usage: cargo run -- get_deleted_project <owner_id> <workspace_id> <project_id>");
            let project_id = args.get(4).expect("Usage: cargo run -- get_deleted_project <owner_id> <workspace_id> <project_id>");
            test_api::test_get_deleted_project(owner_id, workspace_id, project_id).await;
        }

        _ => {
            println!("Unknown test: '{}'. Run without arguments to see available tests.", test_name);
        }
    }
}

