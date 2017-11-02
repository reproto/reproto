use bar::_1_0_0 as bar;
use bar::_2_0_0 as bar2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
  #[serde(skip_serializing_if="Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  other: Option<bar::Other>,
  #[serde(skip_serializing_if="Option::is_none")]
  other2: Option<bar2::Other>,
}
