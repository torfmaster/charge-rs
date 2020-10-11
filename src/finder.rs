use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub eevalue: usize,
    pub epochtime: usize,
}

#[derive(Debug, Deserialize)]
pub struct EEApiResponse {
    pub forecast: Vec<Forecast>,
}

pub fn find_closest_set(forecast: &EEApiResponse) -> Option<&Forecast> {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs_since_epoch = since_the_epoch.as_secs();

    forecast.forecast.iter().min_by(|x, y| {
        (x.epochtime as isize - secs_since_epoch as isize)
            .abs()
            .cmp(&(y.epochtime as isize - secs_since_epoch as isize).abs())
    })
}
