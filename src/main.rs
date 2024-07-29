#![allow(dead_code)]
mod blizz;
mod creatures;
mod oauth;
use blizz::utils::Region;
use std::env;

#[tokio::main]
async fn main() {
    let site_client_id = env::var("BZ_CLIENT_ID").unwrap();
    let site_secret = env::var("BZ_API_KEY").unwrap();
    let token = oauth::token::get_access_token(Region::US).await;
    println!("{:?}", token);

    println!("{}", site_client_id);
    println!("Hello, world!");
}
