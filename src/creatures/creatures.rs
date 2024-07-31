use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Languages;
use crate::blizz::utils::Region;
use crate::error::base_error::BaseError;
use reqwest::RequestBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatureSearchResponse {
    page: u32,
    #[serde(alias = "pageSize")]
    page_size: u32,
    #[serde(alias = "maxPageSize")]
    max_page_size: u32,
    #[serde(alias = "pageCount")]
    page_count: u32,
    results: Vec<CreatureObject>,
}
#[derive(Deserialize)]
pub struct CreatureObject {
    key: Key,
    data: CreatureData,
}
#[derive(Deserialize)]
pub struct CreatureData {
    creature_displays: Vec<CreatureDisplay>,
    is_tameable: bool,
    name: Name,
    id: u32,
    #[serde(alias = "type")]
    kind: CreatureKind,
    family: Option<CreatureFamily>,
}
#[derive(Deserialize)]
pub struct CreatureDisplay {
    key: Option<Key>,
    id: u32,
}
#[derive(Deserialize)]
pub struct Id {
    id: u32,
}
#[derive(Deserialize)]
pub struct Name {
    #[serde(alias = "it_IT")]
    it: Option<String>,
    #[serde(alias = "ru_RU")]
    ru: Option<String>,
    #[serde(alias = "en_GB")]
    gb: Option<String>,
    #[serde(alias = "zh_TW")]
    tw: Option<String>,
    #[serde(alias = "ko_KR")]
    kr: Option<String>,
    #[serde(alias = "en_US")]
    us: Option<String>,
    #[serde(alias = "es_MX")]
    mx: Option<String>,
    #[serde(alias = "pt_BR")]
    br: Option<String>,
    #[serde(alias = "es_ES")]
    es: Option<String>,
    #[serde(alias = "zh_CN")]
    cn: Option<String>,
    #[serde(alias = "fr_FR")]
    fr: Option<String>,
    #[serde(alias = "de_DE")]
    de: Option<String>,
}
#[derive(Deserialize)]
pub struct CreatureKind {
    name: Name,
    id: u32,
}
#[derive(Deserialize)]
pub struct CreatureFamily {
    name: Name,
    id: u32,
}
#[derive(Deserialize)]
pub struct Key {
    href: String,
}
#[derive(Deserialize)]
pub struct Creature {
    href: String,
}
#[derive(Deserialize)]
pub struct Links {
    href: String,
}

pub fn get_creature_image() {}
pub fn get_creature_family_index() {}
pub fn get_creature_family() {}
pub fn get_creature_type_index() {}
pub fn get_creature_type() {}
pub async fn get_creature() {}
//     client: Client,
//     token: String,
//     id: u32,
//     namespace: &str,
//     region: Region,
//     locale: String,
// ) -> Result<Creature, BaseError> {
//     let mut base_url = generate_region_hostname(region);
//     base_url.push_str(&format!("/data/wow/creature/{id}"));
//     let request = client
//         .get(base_url)
//         .query(&[("namespace", namespace)])
//         .bearer_auth(token);
//     match request.send().await {
//         Ok(res) => match res.text().await {
//             Ok(t) => match serde_json::from_str(&t) {
//                 Ok(creature_response) => Ok(creature_response),
//                 Err(e) => {
//                     println!("{:?}", e);
//                     Err(BaseError::UnableToParseJson)
//                 }
//             },
//             Err(e) => {
//                 println!("{:?}", e);
//                 Err(BaseError::UnableToGetText)
//             }
//         },
//         Err(e) => {
//             println!("{:?}", e);
//             Err(BaseError::ServiceUnavailable)
//         }
//     }
// }
pub async fn get_creature_search(
    client: reqwest::Client,
    token: String,
    region: &Region,
    search_term: &str,
    namespace: &str,
    order_by: &str,
    page: &str,
) -> Result<CreatureSearchResponse, BaseError> {
    let mut base_url = generate_region_hostname(region);
    base_url.push_str("/data/wow/search/creature");
    let request = client
        .get(base_url)
        .query(&[
            ("namespace", namespace),
            ("name.en_US", search_term),
            ("orderby", order_by),
            ("_page", page),
        ])
        .bearer_auth(token);
    match request.send().await {
        Ok(res) => match res.text().await {
            Ok(t) => match serde_json::from_str(&t) {
                Ok(creature_response) => Ok(creature_response),
                Err(e) => {
                    println!("{:?}", e);
                    Err(BaseError::UnableToParseJson)
                }
            },
            Err(e) => {
                println!("{:?}", e);
                Err(BaseError::UnableToGetText)
            }
        },
        Err(e) => {
            println!("{:?}", e);
            Err(BaseError::ServiceUnavailable)
        }
    }
}
pub fn get_creature_display_media() {}

pub fn get_creature_family_media() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blizz::utils::Region;
    use crate::oauth::token::get_access_token;
    #[tokio::test]
    async fn test_get_creature_search() {
        let token = get_access_token(&Region::US).await.unwrap();
        let client = reqwest::Client::new();
        match get_creature_search(client, token, &Region::US, "Dragon", "static-us", "id", "1")
            .await
        {
            Ok(_) => {
                assert_eq!(true, true)
            }
            Err(_) => {
                assert_eq!(false, true)
            }
        }
    }
    #[tokio::test]
    async fn test_get_creature_images() {
        let token = get_access_token(&Region::US).await.unwrap();
        let client = reqwest::Client::new();
        match get_creature_search(client, token, &Region::US, "Dragon", "static-us", "id", "1")
            .await
        {
            Ok(search_response) => {
                assert_eq!(true, true)
            }
            Err(_) => {
                assert_eq!(false, true)
            }
        }
    }
    // async fn test_get
}
