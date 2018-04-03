public struct Bar__2_1_0_Other {
  let name21: String
}

public extension Bar__2_1_0_Other {
  static func decode(json: Any) throws -> Bar__2_1_0_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_name21 = json["name21"] else {
      throw SerializationError.missing("name21")
    }

    let name21 = try decode_name(unbox(f_name21, as: String.self), name: "name21")

    return Bar__2_1_0_Other(name21: name21)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["name21"] = self.name21

    return json
  }
}
