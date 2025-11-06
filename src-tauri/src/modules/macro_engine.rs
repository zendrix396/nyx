use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub events: Vec<TimedEvent>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedEvent {
    pub event: rdev::Event,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac<f64>")]
    pub time_since_previous: Duration,
}

