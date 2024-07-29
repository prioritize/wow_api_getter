use std::io;

use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Languages;
use crate::blizz::utils::Region;
use reqwest::RequestBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatureSearchResponse {
    page: u32,
    page_size: u32,
    max_page_size: u32,
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
    creature_displays: Vec<u32>,
    is_tameable: bool,
    name: Name,
    id: Id,
    kind: CreatureKind,
    family: CreatureFamily,
}
#[derive(Deserialize)]
pub struct CreatureDisplay {
    id: Vec<Id>,
}
#[derive(Deserialize)]
pub struct Id {
    id: u32,
}
#[derive(Deserialize)]
pub struct Name {
    #[serde(alias = "it_IT")]
    it: String,
    #[serde(alias = "ru_RU")]
    ru: String,
    #[serde(alias = "en_GB")]
    gb: String,
    #[serde(alias = "zh_TW")]
    tw: String,
    #[serde(alias = "ko_KR")]
    kr: String,
    #[serde(alias = "en_US")]
    us: String,
    #[serde(alias = "es_MX")]
    mx: String,
    #[serde(alias = "pt_BR")]
    br: String,
    #[serde(alias = "es_ES")]
    es: String,
    #[serde(alias = "zh_CN")]
    cn: String,
    #[serde(alias = "fr_FR")]
    fr: String,
    #[serde(alias = "de_DE")]
    de: String,
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
pub fn get_creature_family_index() {}
pub fn get_creature_family() {}
pub fn get_creature_type_index() {}
pub fn get_creature_type() {}
pub fn get_creature() {}
pub async fn get_creature_search(
    client: reqwest::Client,
    token: String,
    region: Region,
    search_term: &str,
    namespace: &str,
    order_by: &str,
    page: &str,
) -> Result<CreatureSearchResponse, io::Error> {
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
    let res = request
        .send()
        .await
        .unwrap()
        .json::<CreatureSearchResponse>()
        .await
        .unwrap();
    println!("{}", res.results.len());
    //println!("Response code is {}", res.unwrap().status());
    todo!()
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
        let token = get_access_token(Region::US).await.unwrap();
        let client = reqwest::Client::new();
        let response =
            get_creature_search(client, token, Region::US, "Dragon", "static-us", "id", "1").await;
    }
}
