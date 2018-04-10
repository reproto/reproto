public struct Test_Entry {
  let tagged: Test_Tagged?
  let required_fields: Test_RequiredFields?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var tagged: Test_Tagged? = Optional.none

    if let value = json["tagged"] {
      tagged = Optional.some(try Test_Tagged.decode(json: value))
    }

    var required_fields: Test_RequiredFields? = Optional.none

    if let value = json["required_fields"] {
      required_fields = Optional.some(try Test_RequiredFields.decode(json: value))
    }

    return Test_Entry(tagged: tagged, required_fields: required_fields)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.tagged {
      json["tagged"] = try value.encode()
    }
    if let value = self.required_fields {
      json["required_fields"] = try value.encode()
    }

    return json
  }
}

public enum Test_Tagged {
  case A(Test_Tagged_A)
  case B(Test_Tagged_B)
  case Bar(Test_Tagged_Bar)
  case Baz(Test_Tagged_Baz)
}

public extension Test_Tagged {
  static func decode(json: Any) throws -> Test_Tagged {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["@type"] as? String, name: "@type")

    switch type {
      case "foo":
        let v = try Test_Tagged_A.decode(json: json)
        return Test_Tagged.A(v)
      case "b":
        let v = try Test_Tagged_B.decode(json: json)
        return Test_Tagged.B(v)
      case "Bar":
        let v = try Test_Tagged_Bar.decode(json: json)
        return Test_Tagged.Bar(v)
      case "Baz":
        let v = try Test_Tagged_Baz.decode(json: json)
        return Test_Tagged.Baz(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .A(let s):
        var json = try s.encode()
        json["@type"] = "foo"
        return json
      case .B(let s):
        var json = try s.encode()
        json["@type"] = "b"
        return json
      case .Bar(let s):
        var json = try s.encode()
        json["@type"] = "Bar"
        return json
      case .Baz(let s):
        var json = try s.encode()
        json["@type"] = "Baz"
        return json
    }
  }
}

public struct Test_Tagged_A {
  let shared: String
}
public extension Test_Tagged_A {
  static func decode(json: Any) throws -> Test_Tagged_A {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    return Test_Tagged_A(shared: shared)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared

    return json
  }
}

public struct Test_Tagged_B {
  let shared: String
}
public extension Test_Tagged_B {
  static func decode(json: Any) throws -> Test_Tagged_B {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    return Test_Tagged_B(shared: shared)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared

    return json
  }
}

public struct Test_Tagged_Bar {
  let shared: String
}
public extension Test_Tagged_Bar {
  static func decode(json: Any) throws -> Test_Tagged_Bar {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    return Test_Tagged_Bar(shared: shared)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared

    return json
  }
}

public struct Test_Tagged_Baz {
  let shared: String
}
public extension Test_Tagged_Baz {
  static func decode(json: Any) throws -> Test_Tagged_Baz {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    return Test_Tagged_Baz(shared: shared)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared

    return json
  }
}

public enum Test_RequiredFields {
  case A(Test_RequiredFields_A)
  case B(Test_RequiredFields_B)
  case C(Test_RequiredFields_C)
}

public extension Test_RequiredFields {
  static func decode(json: Any) throws -> Test_RequiredFields {
    let json = try decode_value(json as? [String: Any])

    let keys = Set(json.keys).subtracting(["shared_ignore"])

    if keys.subtracting(["ignore"]) == ["shared", "a", "b"] {
      return Test_RequiredFields.A(try Test_RequiredFields_A.decode(json: json))
    }

    if keys.subtracting(["ignore"]) == ["shared", "a"] {
      return Test_RequiredFields.B(try Test_RequiredFields_B.decode(json: json))
    }

    if keys.subtracting(["ignore"]) == ["shared", "b"] {
      return Test_RequiredFields.C(try Test_RequiredFields_C.decode(json: json))
    }

    throw SerializationError.invalid("no legal field combinations")
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .A(let s):
        return try s.encode()
      case .B(let s):
        return try s.encode()
      case .C(let s):
        return try s.encode()
    }
  }
}

// Special case: fields shared with other sub-types.
// NOTE: due to rust support through untagged, the types are matched in-order.
public struct Test_RequiredFields_A {
  let shared: String
  let shared_ignore: String?
  let a: String
  let b: String
  let ignore: String?
}
public extension Test_RequiredFields_A {
  static func decode(json: Any) throws -> Test_RequiredFields_A {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    var shared_ignore: String? = Optional.none

    if let value = json["shared_ignore"] {
      shared_ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "shared_ignore"))
    }

    guard let f_a = json["a"] else {
      throw SerializationError.missing("a")
    }

    let a = try decode_name(unbox(f_a, as: String.self), name: "a")

    guard let f_b = json["b"] else {
      throw SerializationError.missing("b")
    }

    let b = try decode_name(unbox(f_b, as: String.self), name: "b")

    var ignore: String? = Optional.none

    if let value = json["ignore"] {
      ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "ignore"))
    }

    return Test_RequiredFields_A(shared: shared, shared_ignore: shared_ignore, a: a, b: b, ignore: ignore)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared
    if let value = self.shared_ignore {
      json["shared_ignore"] = value
    }
    json["a"] = self.a
    json["b"] = self.b
    if let value = self.ignore {
      json["ignore"] = value
    }

    return json
  }
}

public struct Test_RequiredFields_B {
  let shared: String
  let shared_ignore: String?
  let a: String
  let ignore: String?
}
public extension Test_RequiredFields_B {
  static func decode(json: Any) throws -> Test_RequiredFields_B {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    var shared_ignore: String? = Optional.none

    if let value = json["shared_ignore"] {
      shared_ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "shared_ignore"))
    }

    guard let f_a = json["a"] else {
      throw SerializationError.missing("a")
    }

    let a = try decode_name(unbox(f_a, as: String.self), name: "a")

    var ignore: String? = Optional.none

    if let value = json["ignore"] {
      ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "ignore"))
    }

    return Test_RequiredFields_B(shared: shared, shared_ignore: shared_ignore, a: a, ignore: ignore)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared
    if let value = self.shared_ignore {
      json["shared_ignore"] = value
    }
    json["a"] = self.a
    if let value = self.ignore {
      json["ignore"] = value
    }

    return json
  }
}

public struct Test_RequiredFields_C {
  let shared: String
  let shared_ignore: String?
  let b: String
  let ignore: String?
}
public extension Test_RequiredFields_C {
  static func decode(json: Any) throws -> Test_RequiredFields_C {
    let json = try decode_value(json as? [String: Any])

    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")

    var shared_ignore: String? = Optional.none

    if let value = json["shared_ignore"] {
      shared_ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "shared_ignore"))
    }

    guard let f_b = json["b"] else {
      throw SerializationError.missing("b")
    }

    let b = try decode_name(unbox(f_b, as: String.self), name: "b")

    var ignore: String? = Optional.none

    if let value = json["ignore"] {
      ignore = Optional.some(try decode_name(unbox(value, as: String.self), name: "ignore"))
    }

    return Test_RequiredFields_C(shared: shared, shared_ignore: shared_ignore, b: b, ignore: ignore)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["shared"] = self.shared
    if let value = self.shared_ignore {
      json["shared_ignore"] = value
    }
    json["b"] = self.b
    if let value = self.ignore {
      json["ignore"] = value
    }

    return json
  }
}
