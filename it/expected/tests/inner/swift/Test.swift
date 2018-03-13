public struct Test_Entry {
  let a: Test_A?
  let b: Test_A_B?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])
    var a: Test_A? = Optional.none
    if let value = json["a"] {
      a = Optional.some(try Test_A.decode(json: value))
    }
    var b: Test_A_B? = Optional.none
    if let value = json["b"] {
      b = Optional.some(try Test_A_B.decode(json: value))
    }
    return Test_Entry(a: a, b: b)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    if let value = self.a {
      json["a"] = try value.encode()
    }
    if let value = self.b {
      json["b"] = try value.encode()
    }
    return json
  }
}

public struct Test_A {
  let b: Test_A_B
}

public extension Test_A {
  static func decode(json: Any) throws -> Test_A {
    let json = try decode_value(json as? [String: Any])
    guard let f_b = json["b"] else {
      throw SerializationError.missing("b")
    }

    let b = try Test_A_B.decode(json: f_b)
    return Test_A(b: b)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["b"] = try self.b.encode()
    return json
  }
}

public struct Test_A_B {
  let field: String
}

public extension Test_A_B {
  static func decode(json: Any) throws -> Test_A_B {
    let json = try decode_value(json as? [String: Any])
    guard let f_field = json["field"] else {
      throw SerializationError.missing("field")
    }

    let field = try decode_name(unbox(f_field, as: String.self), name: "field")
    return Test_A_B(field: field)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["field"] = self.field
    return json
  }
}
