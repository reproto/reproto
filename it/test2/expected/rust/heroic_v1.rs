use heroic_common as c;
use serde_json as json;
use std::collections;

#[derive(Serialize, Deserialize)]
struct Sampling {
  unit: Option<TimeUnit>,
  size: u32,
  extent: Option<u32>,
}

enum SI {
}

enum TimeUnit {
}

#[derive(Serialize, Deserialize)]
struct Point(u64, double);

#[derive(Serialize, Deserialize)]
struct Event(u64, Option<json::Value>);

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Samples {
  #[serde(rename = "events")]
  Events {
    name: String,
    data: Vec<Event>,
  },

  #[serde(rename = "points")]
  Points {
    name: String,
    data: Vec<Point>,
  },
}

#[derive(Serialize, Deserialize)]
struct Query {
  query: Option<String>,
  aggregation: Option<Aggregation>,
  date: Option<c::Date>,
  parameters: Option<collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Duration {
  #[serde(rename = "absolute")]
  Absolute {
    start: u64,
    end: u64,
  },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Aggregation {
  #[serde(rename = "average")]
  Average {
    sampling: Option<Sampling>,
    size: Option<Duration>,
    extent: Option<Duration>,
  },

  #[serde(rename = "chain")]
  Chain {
    chain: Vec<Aggregation>,
  },

  #[serde(rename = "sum")]
  Sum {
    sampling: Option<Sampling>,
    size: Option<Duration>,
    extent: Option<Duration>,
  },
}

enum ComplexEnum {
}

enum Complex21 {
}
