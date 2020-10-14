use std::{convert::TryInto, time::Duration};

mod create_notification;
mod ee_api;
mod finder;
mod read_battery;
mod thinkpad_load_state;

use async_std::task;
use read_battery::BatteryStatus;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "93051")]
    zip: String,

    #[structopt(short, long, default_value = "80")]
    ee_critical: usize,

    #[structopt(short, long, default_value = "40")]
    battery_critical: usize,

    #[structopt(short, long, default_value = "300")]
    check_interval_secs: usize,

    #[structopt(short, long)]
    use_thinkpad_api: bool,
}

#[cfg(test)]
impl Default for Opt {
    fn default() -> Self {
        Opt {
            zip: "93051".into(),
            ee_critical: 80,
            battery_critical: 40,
            check_interval_secs: 300,
            use_thinkpad_api: false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opt = Opt::from_args();
    loop {
        let _ = send_notification(&opt).await;
        task::sleep(Duration::from_secs(
            opt.check_interval_secs.try_into().unwrap(),
        ))
        .await;
    }
}

#[derive(PartialEq, Debug)]
enum Notification {
    Plugin,
    Plugout,
}

async fn send_notification(opt: &Opt) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = ee_api::query_ee_api(&opt.zip).await?;
    let closest = finder::find_closest_set(&result).unwrap();
    let ee_value = closest.eevalue;

    let status = read_battery::get_status().await?;

    let ratio = read_battery::get_fill_ratio().await?;

    let required_action = create_notification(ee_value, ratio, status, &opt);

    match required_action {
        Some(Notification::Plugin) => create_notification::send("Plug in".into()).await?,
        Some(Notification::Plugout) => create_notification::send("Plug out".into()).await?,
        None => (),
    }

    if opt.use_thinkpad_api {
        match required_action {
            Some(Notification::Plugin) => {
                println!("a {:?}", thinkpad_load_state::start_charging().await)
            }
            Some(Notification::Plugout) => println!(
                "a {:?}",
                thinkpad_load_state::stop_charging(opt.battery_critical).await
            ),
            None => (),
        }
    }

    Ok(())
}

fn create_notification(
    ee_value: usize,
    ratio: usize,
    status: BatteryStatus,
    opt: &Opt,
) -> Option<Notification> {
    if ee_value < opt.ee_critical {
        if ratio < opt.battery_critical {
            match status {
                BatteryStatus::Charging | BatteryStatus::Full => None,
                _ => Some(Notification::Plugin),
            }
        } else {
            match status {
                BatteryStatus::Charging | BatteryStatus::Full => Some(Notification::Plugout),
                _ => None,
            }
        }
    } else {
        match status {
            BatteryStatus::Charging | BatteryStatus::Full => None,
            BatteryStatus::Discharging => Some(Notification::Plugin),
        }
    }
}

#[cfg(test)]
mod test {
    use super::create_notification;
    use super::Notification;
    use crate::BatteryStatus;
    use crate::Opt;

    #[test]
    pub fn asks_to_plug_in_if_enough_ee() {
        let notification = create_notification(81, 90, BatteryStatus::Discharging, &Opt::default());
        assert_eq!(notification, Some(Notification::Plugin));
    }

    #[test]
    pub fn asks_to_plug_out_if_enough_battery_and_no_ee() {
        let notification = create_notification(40, 90, BatteryStatus::Charging, &Opt::default());
        assert_eq!(notification, Some(Notification::Plugout));
    }

    #[test]
    pub fn asks_to_plug_out_if_still_enough_battery_and_no_ee() {
        let notification = create_notification(40, 50, BatteryStatus::Charging, &Opt::default());
        assert_eq!(notification, Some(Notification::Plugout));
    }

    #[test]
    pub fn does_nothing_if_still_enough_battery_and_no_ee_and_plugged_out() {
        let notification = create_notification(40, 50, BatteryStatus::Discharging, &Opt::default());
        assert_eq!(notification, None);
    }
    #[test]
    pub fn asks_to_plug_in_if_enough_battery_and_no_ee_but_energy_critical() {
        let notification = create_notification(40, 20, BatteryStatus::Discharging, &Opt::default());
        assert_eq!(notification, Some(Notification::Plugin));
    }
}
