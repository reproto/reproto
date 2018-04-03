public struct Foo__4_0_0_Thing: Codable {
  let name: String?
  let other: Bar__1_0_0_Other?
  let other2: Bar__2_0_0_Other?
  let other21: Bar__2_1_0_Other?

  enum CodingKeys: String, CodingKey {
    case name = "name"
    case other = "other"
    case other2 = "other2"
    case other21 = "other21"
  }
}
