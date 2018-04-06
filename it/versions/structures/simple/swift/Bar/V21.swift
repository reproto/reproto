public struct Bar_V21_Other {
  let name21: String
}

public extension Bar_V21_Other {
  static func decode(json: Any) throws -> Bar_V21_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_name21 = json["name21"] else {
      throw SerializationError.missing("name21")
    }

    let name21 = try decode_name(unbox(f_name21, as: String.self), name: "name21")

    return Bar_V21_Other(name21: name21)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["name21"] = self.name21

    return json
  }
}
