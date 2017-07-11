use heroic::common as c;
use serde_json as json;
use std::collections;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sampling {
    #[serde(skip_serializing_if="Option::is_none")]
    unit: Option<TimeUnit>,
    size: u32,
    #[serde(skip_serializing_if="Option::is_none")]
    extent: Option<u32>,
}

pub enum SI {
}

pub enum TimeUnit {
}

#[derive(Serialize, Deserialize, Debug)]
struct Point(u64, f64);

#[derive(Serialize, Deserialize, Debug)]
struct Event(u64, Option<json::Value>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Samples {
    #[serde(rename = "events")]
    Events { name: String, data: Vec<Event> },

    #[serde(rename = "points")]
    Points { name: String, data: Vec<Point> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    #[serde(skip_serializing_if="Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    aggregation: Option<Aggregation>,
    #[serde(skip_serializing_if="Option::is_none")]
    date: Option<c::Date>,
    #[serde(skip_serializing_if="Option::is_none")]
    parameters: Option<collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Duration {
    #[serde(rename = "absolute")]
    Absolute { start: u64, end: u64 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Aggregation {
    #[serde(rename = "average")]
    Average {
        #[serde(skip_serializing_if="Option::is_none")]
        sampling: Option<Sampling>,
        #[serde(skip_serializing_if="Option::is_none")]
        size: Option<Duration>,
        #[serde(skip_serializing_if="Option::is_none")]
        extent: Option<Duration>,
    },

    #[serde(rename = "chain")]
    Chain { chain: Vec<Aggregation> },

    #[serde(rename = "sum")]
    Sum {
        #[serde(skip_serializing_if="Option::is_none")]
        sampling: Option<Sampling>,
        #[serde(skip_serializing_if="Option::is_none")]
        size: Option<Duration>,
        #[serde(skip_serializing_if="Option::is_none")]
        extent: Option<Duration>,
    },
}

pub enum ComplexEnum {
}

pub enum Complex21 {
}
