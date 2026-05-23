mod test_api;
use tokio;

//use the test modules here 

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    test_api::test_signup().await;
    test_api::test_login().await;
}
