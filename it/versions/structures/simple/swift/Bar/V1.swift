public struct Bar_V1_Other {
  let name: String
}

public extension Bar_V1_Other {
  static func decode(json: Any) throws -> Bar_V1_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_name = json["name"] else {
      throw SerializationError.missing("name")
    }

    let name = try decode_name(unbox(f_name, as: String.self), name: "name")

    return Bar_V1_Other(name: name)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["name"] = self.name

    return json
  }
}
