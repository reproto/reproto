public struct Foo_V4_Thing: Codable {
  let name: String?
  let other: Bar_V1_Other?
  let other2: Bar_V20_Other?
  let other21: Bar_V21_Other?

  enum CodingKeys: String, CodingKey {
    case name = "name"
    case other = "other"
    case other2 = "other2"
    case other21 = "other21"
  }
}
