#![allow(dead_code)]
mod ah;
mod blizz;
mod connected_realm;
mod creatures;
mod error;
mod items;
mod oauth;
mod wow;
use crate::creatures::creature::CreatureResponse;
use crate::wow::game_data_request::GDRequest;
use blizz::utils::Region;
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    let site_client_id = env::var("BZ_CLIENT_ID").unwrap();
    let site_secret = env::var("BZ_API_KEY").unwrap();
    let token = oauth::token::get_access_token(&Region::US).await.unwrap();
    let client = Client::new();
    let region = Region::US;
    let c_r = CreatureResponse::get(
        client,
        &token,
        &region,
        42722,
        "static-us",
        "en_US",
        "creature",
    )
    .await
    .unwrap();
    println!("{:?}", token);

    println!("{}", site_client_id);
    println!("Hello, world!");
}
