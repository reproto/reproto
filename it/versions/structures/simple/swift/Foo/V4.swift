public struct Foo_V4_Thing {
  let name: String?
  let other: Bar_V1_Other?
  let other2: Bar_V20_Other?
  let other21: Bar_V21_Other?
}

public extension Foo_V4_Thing {
  static func decode(json: Any) throws -> Foo_V4_Thing {
    let json = try decode_value(json as? [String: Any])

    var name: String? = Optional.none

    if let value = json["name"] {
      name = Optional.some(try decode_name(unbox(value, as: String.self), name: "name"))
    }

    var other: Bar_V1_Other? = Optional.none

    if let value = json["other"] {
      other = Optional.some(try Bar_V1_Other.decode(json: value))
    }

    var other2: Bar_V20_Other? = Optional.none

    if let value = json["other2"] {
      other2 = Optional.some(try Bar_V20_Other.decode(json: value))
    }

    var other21: Bar_V21_Other? = Optional.none

    if let value = json["other21"] {
      other21 = Optional.some(try Bar_V21_Other.decode(json: value))
    }

    return Foo_V4_Thing(name: name, other: other, other2: other2, other21: other21)
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
    if let value = self.other21 {
      json["other21"] = try value.encode()
    }

    return json
  }
}
