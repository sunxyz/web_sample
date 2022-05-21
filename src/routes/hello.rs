use super::*;

pub async fn root() -> &'static str {
    REPO.save(&Demo{
        id: Some("1".to_string()),
        name: Some("test".to_string()),
    }).await.unwrap();
    log::info!("{}", "Hello, world!");
    "Hello, World!"
}