use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;

#[derive(Debug)]
pub enum BatteryStatus {
    Full,
    Charging,
    Discharging,
    Unknown,
}

pub async fn get_status() -> io::Result<BatteryStatus> {
    let file = File::open("/sys/class/power_supply/BAT0/status").await?;
    let mut result = String::new();
    BufReader::new(file).read_line(&mut result).await?;
    Ok(parse_battery_status(result))
}

pub fn parse_battery_status(input: String) -> BatteryStatus {
    match input.trim() {
        "Full" => BatteryStatus::Full,
        "Charging" => BatteryStatus::Charging,
        "Discharging" => BatteryStatus::Discharging,
        _ => BatteryStatus::Unknown,
    }
}

pub async fn get_fill_ratio() -> io::Result<usize> {
    let file = File::open("/sys/class/power_supply/BAT0/capacity").await?;
    let mut result = String::new();
    BufReader::new(file).read_line(&mut result).await?;
    let fill: usize = result.trim().parse().unwrap();
    Ok(fill)
}
