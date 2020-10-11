use crate::finder::EEApiResponse;

pub async fn query_ee_api(
    zip: &String,
) -> Result<EEApiResponse, Box<dyn std::error::Error + Send + Sync>> {
    reqwest::get(format!("https://api.corrently.io/core/gsi?zip={}", zip).as_str())
        .await?
        .json::<EEApiResponse>()
        .await
        .map_err(|err| err.into())
}
