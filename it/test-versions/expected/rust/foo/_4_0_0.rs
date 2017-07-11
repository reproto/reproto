use bar::_1_0_0 as bar;
use bar::_2_0_0 as bar2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
    name: String,
    other: bar::Other,
    other2: bar2::Other,
}
