use async_std::fs::File;
use async_std::prelude::*;

pub async fn stop_charging(min: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stop_theshold =
        File::open("/sys/class/power_supply/BAT0/charge_stop_threshold").await?;

    stop_theshold.write_all(format!("{}", min).as_bytes());

    let mut start_threshold =
        File::open("/sys/class/power_supply/BAT0/charge_start_threshold").await?;

    start_threshold.write_all(format!("{}", min - 4).as_bytes());
    Ok(())
}

pub async fn start_charging() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stop_theshold =
        File::open("/sys/class/power_supply/BAT0/charge_stop_threshold").await?;

    stop_theshold.write_all("100".as_bytes());

    let mut start_threshold =
        File::open("/sys/class/power_supply/BAT0/charge_start_threshold").await?;

    start_threshold.write_all("96".as_bytes());

    Ok(())
}
