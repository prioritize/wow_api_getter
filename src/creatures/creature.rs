use serde::Deserialize;

use crate::wow::game_data_request::GDRequest;

#[derive(Deserialize)]
struct CreatureResponse {
    #[serde(alias = "_links")]
    links: Links,
    id: u32,
    name: String,
    creature_type: CreatureType,
    creature_family: CreatureFamily,
    creature_display: Vec<CreatureDisplay>,
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
struct CreatureDisplay {
    key: Key,
    id: u32,
}
#[derive(Deserialize)]
struct Links {
    #[serde(alias = "self")]
    this: Key,
}
#[derive(Deserialize)]
struct Key {
    href: String,
}
impl GDRequest for CreatureResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blizz::utils::Region;
    use crate::oauth::token::get_access_token;
    use crate::wow::game_data_request::GDRequest;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_creature() {
        let client = Client::new();
        let region = Region::US;
        let token = get_access_token(&region).await.unwrap();

        match CreatureResponse::get(
            client,
            token,
            &region,
            42722,
            "static-us",
            "en_US",
            "creature",
        )
        .await
        {
            Ok(creature_response) => {
                // let reqwest::get(creature_response.links.this.href)..await;
                assert_eq!(0, 0);
            }
            Err(e) => {
                println!("{:?}", e);
                assert_eq!(0, 1);
            }
        }
    }
}
