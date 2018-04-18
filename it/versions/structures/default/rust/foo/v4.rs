use bar::v1 as bar;
use bar::v2_0 as bar2;
use bar::v2_1 as bar21;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thing {
  #[serde(skip_serializing_if="Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub other: Option<bar::Other>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub other2: Option<bar2::Other>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub other21: Option<bar21::Other>,
}
