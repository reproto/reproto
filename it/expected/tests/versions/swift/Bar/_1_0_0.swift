public struct Bar__1_0_0_Other {
  let name: String
}

public extension Bar__1_0_0_Other {
  static func decode(json: Any) throws -> Bar__1_0_0_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_name = json["name"] else {
      throw SerializationError.missing("name")
    }

    let name = try decode_name(unbox(f_name, as: String.self), name: "name")

    return Bar__1_0_0_Other(name: name)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["name"] = self.name

    return json
  }
}
