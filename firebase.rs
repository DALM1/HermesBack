use reqwest::Client;
use serde_json::json;
use std::env;
use dotenv::dotenv;

const FIREBASE_COLLECTION: &str = "rooms";

pub async fn save_message(room: &str, user: &str, content: &str) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let project_id = env::var("FIREBASE_PROJECT_ID").expect("FIREBASE_PROJECT_ID not set");
    let database_url = env::var("FIREBASE_DATABASE_URL").expect("FIREBASE_DATABASE_URL not set");

    let client = Client::new();
    let message = json!({
        "user": user,
        "content": content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let url = format!("{}/{}/{}/messages.json", database_url, FIREBASE_COLLECTION, room);

    client.post(&url)
        .json(&message)
        .send()
        .await?;

    Ok(())
}

pub async fn get_messages(room: &str) -> Result<serde_json::Value, reqwest::Error> {
    dotenv().ok();
    let database_url = env::var("FIREBASE_DATABASE_URL").expect("FIREBASE_DATABASE_URL not set");

    let url = format!("{}/{}/{}/messages.json", database_url, FIREBASE_COLLECTION, room);
    let response = reqwest::get(&url).await?.json().await?;
    Ok(response)
}
