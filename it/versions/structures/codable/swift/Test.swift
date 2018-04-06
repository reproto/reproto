public struct Test_Entry: Codable {
  let thing: Foo_V4_Thing?

  enum CodingKeys: String, CodingKey {
    case thing = "thing"
  }
}
