use serde::Deserialize;
#[derive(Deserialize)]
pub struct Links {
    #[serde(alias = "self")]
    this: Key,
}
