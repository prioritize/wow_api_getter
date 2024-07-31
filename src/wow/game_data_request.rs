use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Region;
use crate::error::base_error::BaseError;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;

pub trait GDRequest {
    async fn get(
        client: Client,
        token: &str,
        region: &Region,
        id: u32,
        namespace: &str,
        locale: &str,
        kind: &str,
    ) -> Result<Self, BaseError>
    where
        Self: Sized + for<'de> Deserialize<'de>,
    {
        let url = generate_region_hostname(&region);
        let url = format!("{url}/data/wow/{kind}/{id}");
        println!("{}", &url);
        match client
            .get(url)
            .query(&[("namespace", namespace), ("locale", locale)])
            .bearer_auth(token)
            .send()
            .await
        {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let text = response.text().await.unwrap();
                    println!("{}", text);
                    match serde_json::from_str(&text) {
                        Ok(parsed) => Ok(parsed),
                        Err(e) => {
                            println!("{:?}", e);
                            Err(BaseError::UnableToParseJson)
                        }
                    }
                }
                // match response.json().await {
                //     Ok(parsed) => Ok(parsed),
                //     Err(_) => Err(BaseError::UnableToParseJson),
                // },
                StatusCode::NOT_FOUND => Err(BaseError::NotFound),
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
