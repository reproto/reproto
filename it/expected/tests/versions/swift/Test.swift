public struct Test_Entry {
  let thing: Foo__4_0_0_Thing?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var thing: Foo__4_0_0_Thing? = Optional.none

    if let value = json["thing"] {
      thing = Optional.some(try Foo__4_0_0_Thing.decode(json: value))
    }

    return Test_Entry(thing: thing)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    if let value = self.thing {
      json["thing"] = try value.encode()
    }
    return json
  }
}
