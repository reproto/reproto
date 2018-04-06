public struct Bar_V1_Other: Codable {
  let name: String

  enum CodingKeys: String, CodingKey {
    case name = "name"
  }
}
