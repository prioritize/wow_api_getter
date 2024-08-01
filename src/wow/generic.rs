use serde::Deserialize;
#[derive(Deserialize)]
pub struct Links {
    #[serde(alias = "self")]
    self_link: Key,
}
#[derive(Deserialize)]
pub struct Key {
    pub href: String,
}
