use reqwest::Client;

use crate::blizz::utils::Region;

trait GDRequest<T> {
    async fn get(client: Client, token: String, region: Region, url: String);
}
