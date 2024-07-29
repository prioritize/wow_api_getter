use std::fmt::Display;
pub enum Region {
    US,
    EU,
    Korea,
    Taiwan,
}
pub enum Language {
    Italian,
    Russian,
    BritishEnglish,
    Taiwanese,
    Korean,
    USEnglish,
    MXSpanish,
    Portugese,
    Spanish,
    Chinese,
    French,
    German,
}
pub struct Languages {
    it: String,
    ru: String,
    gb: String,
    tw: String,
    kr: String,
    us: String,
    mx: String,
    br: String,
    es: String,
    cn: String,
    fr: String,
    de: String,
}
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::US => write!(f, "https://us"),
            Region::EU => write!(f, "https://eu"),
            Region::Korea => write!(f, "https://kr"),
            Region::Taiwan => write!(f, "https://tw"),
        }
    }
}
pub fn generate_region_hostname(region: Region) -> String {
    region.to_string() + ".api.blizzard.com"
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth::token::get_access_token;

    #[test]
    fn print_us_hostname() {
        assert_eq!(
            "https://us.api.blizzard.com",
            generate_region_hostname(Region::US)
        )
    }
    #[test]
    fn print_eu_hostname() {
        assert_eq!(
            "https://eu.api.blizzard.com",
            generate_region_hostname(Region::EU)
        )
    }
    #[test]
    fn print_kr_hostname() {
        assert_eq!(
            "https://kr.api.blizzard.com",
            generate_region_hostname(Region::Korea)
        )
    }
    #[test]
    fn print_tw_hostname() {
        assert_eq!(
            "https://tw.api.blizzard.com",
            generate_region_hostname(Region::Taiwan)
        )
    }
    #[test]
    fn get_token() {
        let _ = get_access_token(Region::US);
        assert_eq!(1 + 1, 2);
    }
}
