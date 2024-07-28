mod blizz;
mod creatures;
mod oauth;
use std::env;
fn main() {
    let site_client_id = env::var("BZ_CLIENT_ID").unwrap();
    let site_secret = env::var("BZ_API_KEY").unwrap();
    let res = reqwest::get("google.com");
    println!("{}", site_client_id);
    println!("Hello, world!");
}
