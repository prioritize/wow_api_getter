use crate::{
    blizz::utils::Region,
    error::base_error::BaseError,
    oauth::token::get_access_token,
    wow::generic::{Key, Links},
};
use reqwest::{Client, Method};
use serde::Deserialize;

#[derive(Deserialize)]
struct ConnectedRealmIndex {
    #[serde(alias = "_links")]
    links: Links,
    connected_realms: Vec<Key>,
}
struct ConnectedRealmUrl {
    href: String,
}

#[derive(Deserialize)]
struct ConnectedRealm {
    #[serde(alias = "_links")]
    links: Links,
    id: u32,
    has_queue: bool,
    status: ConnectedRealmStatus,
    population: ConnectedRealmStatus,
    realms: Vec<RealmRegion>,
    mythic_leaderboards: Key,
    auctions: Key,
}
#[derive(Deserialize)]
struct RealmRegion {
    key: Key,
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct Realm {
    id: u32,
    region: RealmRegion,
    name: String,
    connected_realm: Key,
    category: String,
    locale: String,
    timezone: String,
    #[serde(alias = "type")]
    kind: ConnectedRealmStatus,
    is_tournament: bool,
    slug: String,
}

#[derive(Deserialize)]
struct ConnectedRealmStatus {
    #[serde(alias = "type")]
    kind: String,
    name: String,
}

impl ConnectedRealmUrl {
    fn new(k: &Key) -> Self {
        ConnectedRealmUrl {
            href: k.href.clone(),
        }
    }
    async fn get_realms(region: &Region) -> Result<Vec<ConnectedRealmUrl>, BaseError> {
        let realm_index = ConnectedRealmIndex::get(region).await.unwrap();
        Ok(realm_index
            .connected_realms
            .iter()
            .map(|r| ConnectedRealmUrl::new(r))
            .collect())
    }
}

impl ConnectedRealmIndex {
    async fn get(region: &Region) -> Result<ConnectedRealmIndex, BaseError> {
        let url = format!("{region}.api.blizzard.com/data/wow/connected-realm/index");
        let query = [("namespace", "dynamic-us"), ("locale", "en_US")];
        let token = get_access_token(&region).await.unwrap();
        let index: ConnectedRealmIndex = Client::new()
            .request(Method::GET, url)
            .bearer_auth(token)
            .query(&query)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::blizz::utils::Region;

    #[tokio::test]
    async fn test_get_connected_realm_index() {
        let region = Region::US;
        let token = get_access_token(&region).await.unwrap();
        let client = Client::new();
        let index = ConnectedRealmIndex::get(&Region::US).await.unwrap();
        let realms = ConnectedRealmUrl::get_realms(&Region::US).await.unwrap();
        let mut storage = vec![];
        realms.iter().for_each(|r| async {
            storage.push(
                client
                    .request(Method::GET, r.href)
                    .bearer_auth(&token)
                    .send()
                    .await
                    .unwrap()
                    .json::<ConnectedRealm>()
                    .await
                    .unwrap(),
            );
        });
    }
}
