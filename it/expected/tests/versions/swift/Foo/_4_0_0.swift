public struct Foo__4_0_0_Thing {
  let name: String?
  let other: Bar__1_0_0_Other?
  let other2: Bar__2_0_0_Other?
}

public extension Foo__4_0_0_Thing {
  static func decode(json: Any) throws -> Foo__4_0_0_Thing {
    let json = try decode_value(json as? [String: Any])

    var name: String? = Optional.none

    if let value = json["name"] {
      name = Optional.some(try decode_name(unbox(value, as: String.self), name: "name"))
    }

    var other: Bar__1_0_0_Other? = Optional.none

    if let value = json["other"] {
      other = Optional.some(try Bar__1_0_0_Other.decode(json: value))
    }

    var other2: Bar__2_0_0_Other? = Optional.none

    if let value = json["other2"] {
      other2 = Optional.some(try Bar__2_0_0_Other.decode(json: value))
    }

    return Foo__4_0_0_Thing(name: name, other: other, other2: other2)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    if let value = self.name {
      json["name"] = value
    }
    if let value = self.other {
      json["other"] = try value.encode()
    }
    if let value = self.other2 {
      json["other2"] = try value.encode()
    }
    return json
  }
}
