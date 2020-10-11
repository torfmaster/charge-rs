use crate::finder::EEApiResponse;

pub async fn query_ee_api() -> Result<EEApiResponse, Box<dyn std::error::Error + Send + Sync>> {
    reqwest::get("https://api.corrently.io/core/gsi?zip=93051")
        .await?
        .json::<EEApiResponse>()
        .await
        .map_err(|err| err.into())
}
