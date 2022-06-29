public struct Test_Entry {
  // The foo field.
  let foo: Test_Foo?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var foo: Test_Foo? = Optional.none

    if let value = json["foo"] {
      foo = Optional.some(try Test_Foo.decode(json: value))
    }
    return Test_Entry(foo: foo)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.foo {
      json["foo"] = try value.encode()
    }

    return json
  }
}

public struct Test_Foo {
  // The field.
  let field: String
}

public extension Test_Foo {
  static func decode(json: Any) throws -> Test_Foo {
    let json = try decode_value(json as? [String: Any])

    guard let f_field = json["field"] else {
      throw SerializationError.missing("field")
    }

    let field = try decode_name(unbox(f_field, as: String.self), name: "field")
    return Test_Foo(field: field)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["field"] = self.field

    return json
  }
}

public struct Test_Bar {
  // The inner field.
  let field: Test_Bar_Inner
}

public extension Test_Bar {
  static func decode(json: Any) throws -> Test_Bar {
    let json = try decode_value(json as? [String: Any])

    guard let f_field = json["field"] else {
      throw SerializationError.missing("field")
    }

    let field = try Test_Bar_Inner.decode(json: f_field)
    return Test_Bar(field: field)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["field"] = try self.field.encode()

    return json
  }
}

public struct Test_Bar_Inner {
  // The field.
  let field: String
}

public extension Test_Bar_Inner {
  static func decode(json: Any) throws -> Test_Bar_Inner {
    let json = try decode_value(json as? [String: Any])

    guard let f_field = json["field"] else {
      throw SerializationError.missing("field")
    }

    let field = try decode_name(unbox(f_field, as: String.self), name: "field")
    return Test_Bar_Inner(field: field)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["field"] = self.field

    return json
  }
}
