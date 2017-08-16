#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  a: Entry_A,
  b: Entry_A_B,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry_A {
  b: Entry_A_B,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry_A_B {
  field: String,
}
