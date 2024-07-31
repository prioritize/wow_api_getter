use serde::Deserialize;

use crate::wow::game_data_request::GDRequest;

#[derive(Deserialize)]
pub struct CreatureResponse {
    #[serde(alias = "_links")]
    links: Links,
    id: u32,
    name: String,
    #[serde(alias = "type")]
    creature_type: CreatureType,
    #[serde(alias = "family")]
    creature_family: CreatureFamily,
    creature_displays: Vec<CreatureDisplay>,
    is_tameable: bool,
}
#[derive(Deserialize)]
struct CreatureType {
    key: Key,
    name: String,
    id: u32,
}
#[derive(Deserialize)]
struct CreatureFamily {
    key: Key,
    name: String,
    id: u32,
}
#[derive(Deserialize)]
pub struct CreatureDisplay {
    key: Key,
    id: u32,
}
#[derive(Deserialize)]
pub struct Links {
    #[serde(alias = "self")]
    this: Key,
}
#[derive(Deserialize)]
pub struct Key {
    href: String,
}
#[derive(Deserialize)]
pub struct CreatureMedia {
    #[serde(alias = "_links")]
    links: Links,
    assets: Vec<MediaKV>,
    id: u32,
}
#[derive(Deserialize)]
pub struct MediaKV {
    key: String,
    value: String,
}

impl GDRequest for CreatureResponse {}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::blizz::utils::Region;
    use crate::oauth::token::get_access_token;
    use crate::wow::game_data_request::GDRequest;
    use reqwest::{Client, Method};

    #[tokio::test]
    async fn test_get_creature() {
        let client = Client::new();
        let region = Region::US;
        let token = get_access_token(&region).await.unwrap();

        match CreatureResponse::get(
            client.clone(),
            &token,
            &region,
            38046,
            "static-us",
            "en_US",
            "creature",
        )
        .await
        {
            Ok(creature_response) => {
                // let reqwest::get(creature_response.links.this.href)..await;
                let res: CreatureMedia = match client
                    .request(
                        Method::GET,
                        &creature_response.creature_displays[0].key.href,
                    )
                    .bearer_auth(&token)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                {
                    Ok(cm) => cm,
                    Err(_) => {
                        panic!("Bad shit")
                    }
                };
                let res = client
                    .clone()
                    .request(Method::GET, &res.assets[0].value)
                    .bearer_auth(&token)
                    .send()
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                let mut file = std::fs::File::create_new("test_image.png").unwrap();
                file.write_all(&res).unwrap();

                assert_eq!(0, 0);
            }
            Err(e) => {
                println!("{:?}", e);
                assert_eq!(0, 1);
            }
        }
    }
}
