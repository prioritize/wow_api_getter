use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Region;
use oauth2::reqwest::async_http_client;
use oauth2::reqwest::http_client;
use oauth2::TokenResponse;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl};
use std::env;
pub async fn get_access_token(region: Region) -> Result<String, oauth2::url::ParseError> {
    let client_id = env::var("BZ_CLIENT_ID").expect("Unable to find BZ_CLIENT_ID");
    let client_secret = env::var("BZ_API_KEY").expect("Unable to find BZ_API_KEY");
    let auth_url = generate_region_hostname(region);
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new(auth_url.to_string()).expect("failed to parse AuthUrl"),
        Some(
            TokenUrl::new("https://oauth.battle.net/token".to_string())
                .expect("failed to parse TokenUrl"),
        ),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(
        "http://greenpetergames.com/ah".to_string(),
    )?);
    match client
        .exchange_client_credentials()
        .add_scope(Scope::new("read".to_string()))
        .request_async(async_http_client)
        .await
    {
        Ok(tr) => Ok(tr.access_token().secret().to_owned()),
        Err(_) => todo!(),
    }
}
