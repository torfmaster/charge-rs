use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;

#[derive(Debug)]
pub enum BatteryStatus {
    Full,
    Charging,
    Discharging,
}

pub async fn get_status() -> io::Result<BatteryStatus> {
    let file = File::open("/sys/class/power_supply/BAT0/status").await?;
    let mut result = String::new();
    BufReader::new(file).read_line(&mut result).await?;
    Ok(parse_battery_status(result).unwrap())
}

pub fn parse_battery_status(input: String) -> Option<BatteryStatus> {
    match input.trim() {
        "Full" => Some(BatteryStatus::Full),
        "Charging" => Some(BatteryStatus::Charging),
        "Discharging" => Some(BatteryStatus::Discharging),
        _ => None,
    }
}

async fn get_fill_state() -> io::Result<usize> {
    let file = File::open("/sys/class/power_supply/BAT0/charge_now").await?;
    let mut result = String::new();
    BufReader::new(file).read_line(&mut result).await?;
    let fill: usize = result.trim().parse().unwrap();
    Ok(fill)
}

async fn get_max_fill() -> io::Result<usize> {
    let file = File::open("/sys/class/power_supply/BAT0/charge_full").await?;
    let mut result = String::new();
    BufReader::new(file).read_line(&mut result).await?;
    let fill: usize = result.trim().parse().unwrap();
    Ok(fill)
}

pub async fn get_fill_ratio() -> io::Result<usize> {
    let current = get_fill_state().await?;
    let max = get_max_fill().await?;
    Ok(current * 100 / max)
}
