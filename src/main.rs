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
