public struct Bar_V20_Other {
  let name2: String
}

public extension Bar_V20_Other {
  static func decode(json: Any) throws -> Bar_V20_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_name2 = json["name2"] else {
      throw SerializationError.missing("name2")
    }

    let name2 = try decode_name(unbox(f_name2, as: String.self), name: "name2")

    return Bar_V20_Other(name2: name2)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["name2"] = self.name2

    return json
  }
}
