use reqwest::Client;
use crate::blizz::utils::generate_region_hostname;
use crate::blizz::utils::Region;

pub trait GDRequest<T> {
    async fn get(
        client: Client,
        token: String,
        region: Region,
        url: String,
        namespace: String,
        locale: String,
        query_args: Vec<(&str, &str)>,
    ) -> T {
        let base_url = 
    }
}
