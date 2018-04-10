public struct Test_Entry {
  let tagged: Test_Tagged?
  let untagged: Test_Untagged?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var tagged: Test_Tagged? = Optional.none

    if let value = json["tagged"] {
      tagged = Optional.some(try Test_Tagged.decode(json: value))
    }

    var untagged: Test_Untagged? = Optional.none

    if let value = json["untagged"] {
      untagged = Optional.some(try Test_Untagged.decode(json: value))
    }

    return Test_Entry(tagged: tagged, untagged: untagged)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.tagged {
      json["tagged"] = try value.encode()
    }
    if let value = self.untagged {
      json["untagged"] = try value.encode()
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

public enum Test_Untagged {
  case A(Test_Untagged_A)
  case B(Test_Untagged_B)
  case C(Test_Untagged_C)
}

public extension Test_Untagged {
  static func decode(json: Any) throws -> Test_Untagged {
    let json = try decode_value(json as? [String: Any])

    let keys = Set(json.keys).subtracting(["shared_ignore"])

    if keys.subtracting(["ignore"]) == ["shared", "a", "b"] {
      return Test_Untagged.A(try Test_Untagged_A.decode(json: json))
    }

    if keys.subtracting(["ignore"]) == ["shared", "a"] {
      return Test_Untagged.B(try Test_Untagged_B.decode(json: json))
    }

    if keys.subtracting(["ignore"]) == ["shared", "b"] {
      return Test_Untagged.C(try Test_Untagged_C.decode(json: json))
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
public struct Test_Untagged_A {
  let shared: String
  let shared_ignore: String?
  let a: String
  let b: String
  let ignore: String?
}
public extension Test_Untagged_A {
  static func decode(json: Any) throws -> Test_Untagged_A {
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

    return Test_Untagged_A(shared: shared, shared_ignore: shared_ignore, a: a, b: b, ignore: ignore)
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

public struct Test_Untagged_B {
  let shared: String
  let shared_ignore: String?
  let a: String
  let ignore: String?
}
public extension Test_Untagged_B {
  static func decode(json: Any) throws -> Test_Untagged_B {
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

    return Test_Untagged_B(shared: shared, shared_ignore: shared_ignore, a: a, ignore: ignore)
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

public struct Test_Untagged_C {
  let shared: String
  let shared_ignore: String?
  let b: String
  let ignore: String?
}
public extension Test_Untagged_C {
  static func decode(json: Any) throws -> Test_Untagged_C {
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

    return Test_Untagged_C(shared: shared, shared_ignore: shared_ignore, b: b, ignore: ignore)
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
