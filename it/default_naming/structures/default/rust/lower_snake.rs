#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
  foo_bar: String,
}

pub trait Service {
  fn foo_bar();
}
