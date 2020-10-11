use std::time::Duration;

mod create_notification;
mod ee_api;
mod finder;
mod read_battery;

use async_std::task;
use read_battery::BatteryStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        send_notification().await?;
        task::sleep(Duration::from_secs(120)).await;
    }
}

#[derive(PartialEq, Debug)]
enum Notification {
    Plugin,
    Plugout,
}

async fn send_notification() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = ee_api::query_ee_api().await?;
    let closest = finder::find_closest_set(&result).unwrap();
    let ee_value = closest.eevalue;

    let status = read_battery::get_status().await?;

    let ratio = read_battery::get_fill_ratio().await?;

    match create_notification(ee_value, ratio, status) {
        Some(Notification::Plugin) => create_notification::send("Stecker rein".into()).await?,
        Some(Notification::Plugout) => create_notification::send("Stecker raus".into()).await?,
        None => (),
    }

    Ok(())
}

fn create_notification(
    ee_value: usize,
    ratio: usize,
    status: BatteryStatus,
) -> Option<Notification> {
    match ee_value {
        0..=80 => match ratio {
            0..=40 => match status {
                BatteryStatus::Charging | BatteryStatus::Full => None,
                _ => Some(Notification::Plugin),
            },
            _ => match status {
                BatteryStatus::Charging | BatteryStatus::Full => Some(Notification::Plugout),
                _ => None,
            },
        },
        _ => match status {
            BatteryStatus::Charging | BatteryStatus::Full => None,
            BatteryStatus::Discharging => Some(Notification::Plugin),
        },
    }
}

#[cfg(test)]
mod test {
    use super::create_notification;
    use super::Notification;
    use crate::BatteryStatus;
    #[test]
    pub fn asks_to_plug_in_if_enough_ee() {
        let notification = create_notification(81, 90, BatteryStatus::Discharging);
        assert_eq!(notification, Some(Notification::Plugin));
    }

    #[test]
    pub fn asks_to_plug_out_if_enough_battery_and_no_ee() {
        let notification = create_notification(40, 90, BatteryStatus::Charging);
        assert_eq!(notification, Some(Notification::Plugout));
    }

    #[test]
    pub fn asks_to_plug_in_if_enough_battery_and_no_ee_but_energy_critical() {
        let notification = create_notification(40, 20, BatteryStatus::Discharging);
        assert_eq!(notification, Some(Notification::Plugin));
    }
}
