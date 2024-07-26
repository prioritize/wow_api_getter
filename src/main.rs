use std::{env, fmt::Display};
enum Region {
    US,
    EU,
    Korea,
    Taiwan,
}
fn main() {
    let api_key = env::var("BZ_API_KEY").unwrap();
    let res = reqwest::get("google.com");
    println!("{}", api_key);
    println!("Hello, world!");
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::US => write!(f, "us"),
            Region::EU => write!(f, "eu"),
            Region::Korea => write!(f, "kr"),
            Region::Taiwan => write!(f, "tw"),
        }
    }
}
fn generate_region_hostname(region: Region) -> String {
    region.to_string() + ".api.blizzard.com"
}
fn get_access_token(
    region: Region,
    response_type: &str,
    client_id: &str,
    redirect_uri: &str,
) -> String {
    use oauth2::basic::BasicClient;
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string())?,
        Some(TokenUrl::new("http://token".to_string())?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)?;

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_us_hostname() {
        assert_eq!("us.api.blizzard.com", generate_region_hostname(Region::US))
    }
    #[test]
    fn print_eu_hostname() {
        assert_eq!("eu.api.blizzard.com", generate_region_hostname(Region::EU))
    }
    #[test]
    fn print_kr_hostname() {
        assert_eq!(
            "kr.api.blizzard.com",
            generate_region_hostname(Region::Korea)
        )
    }
    #[test]
    fn print_tw_hostname() {
        assert_eq!(
            "tw.api.blizzard.com",
            generate_region_hostname(Region::Taiwan)
        )
    }
}
