use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Region;
use crate::error::base_error::BaseError;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;

pub trait GDRequest {
    async fn get(
        client: Client,
        token: String,
        region: Region,
        id: u32,
        namespace: String,
        locale: String,
        kind: &str,
    ) -> Result<Self, BaseError>
    where
        T: for<'de> Deserialize<'de>,
        Self: Sized + for<'de> Deserialize<'de>,
    {
        let url = generate_region_hostname(region);
        let url = format!("{url}/{kind}/{id}");
        match client
            .get(url)
            .query(&[("namespace", namespace), ("locale", locale)])
            .bearer_auth(token)
            .send()
            .await
        {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json().await {
                    Ok(parsed) => Ok(parsed),
                    Err(_) => Err(BaseError::UnableToParseJson),
                },
                _ => Err(BaseError::ServiceUnavailable),
            },
            Err(_) => todo!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
