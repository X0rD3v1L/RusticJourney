use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde::Deserialize;

const BASE: &str = "https://gorest.co.in/public/v2";
const CONCURRENCY: usize = 10;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
    gender: String,
    status: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Post {
    id: u64,
    user_id: u64,
    title: String,
    body: String,
}

async fn fetch_users(client: &Client) -> Result<Vec<User>, reqwest::Error> {
    let url = format!("{}/users", BASE);
    client.get(url).send().await?.json::<Vec<User>>().await
}

async fn fetch_posts(client: &Client, user_id: u64) -> Result<Vec<Post>, reqwest::Error> {
    let url = format!("{}/users/{}/posts", BASE, user_id);
    client.get(url).send().await?.json::<Vec<Post>>().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let threshold = 2;

    let users = fetch_users(&client).await?;

    let results = stream::iter(users.clone())
        .map(|user| {
            let client = client.clone();
            async move {
                match fetch_posts(&client, user.id).await {
                    Ok(posts) => Some((user, posts.len())),
                    Err(_) => None,
                }
            }
        })
        .buffer_unordered(CONCURRENCY)
        .filter_map(|x| async move { x })
        .collect::<Vec<_>>()
        .await;

    println!("Users with more than {} posts:", threshold);

    for (user, count) in results.into_iter().filter(|(_, c)| *c >= threshold) {
        println!("{} (id: {}) -> {} posts", user.name, user.id, count);
    }

    Ok(())
}