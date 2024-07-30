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
    use crate::oauth::token;
    use crate::wow::game_data_request::GDRequest;
    use reqwest::Client;
    fn test_get_creature() {
        let client = Client::new();
    }
}
