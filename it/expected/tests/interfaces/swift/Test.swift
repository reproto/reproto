public enum Test_Entry {
  case A(Test_Entry_A)
  case B(Test_Entry_B)
  case Bar(Test_Entry_Bar)
  case Baz(Test_Entry_Baz)
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["@type"] as? String, name: "@type")

    switch type {
      case "foo":
        let v = try Test_Entry_A.decode(json: json)
        return Test_Entry.A(v)
      case "b":
        let v = try Test_Entry_B.decode(json: json)
        return Test_Entry.B(v)
      case "Bar":
        let v = try Test_Entry_Bar.decode(json: json)
        return Test_Entry.Bar(v)
      case "Baz":
        let v = try Test_Entry_Baz.decode(json: json)
        return Test_Entry.Baz(v)
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

public struct Test_Entry_A {
  let shared: String
}
public extension Test_Entry_A {
  static func decode(json: Any) throws -> Test_Entry_A {
    let json = try decode_value(json as? [String: Any])
    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")
    return Test_Entry_A(shared: shared)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["shared"] = self.shared
    return json
  }
}

public struct Test_Entry_B {
  let shared: String
}
public extension Test_Entry_B {
  static func decode(json: Any) throws -> Test_Entry_B {
    let json = try decode_value(json as? [String: Any])
    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")
    return Test_Entry_B(shared: shared)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["shared"] = self.shared
    return json
  }
}

public struct Test_Entry_Bar {
  let shared: String
}
public extension Test_Entry_Bar {
  static func decode(json: Any) throws -> Test_Entry_Bar {
    let json = try decode_value(json as? [String: Any])
    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")
    return Test_Entry_Bar(shared: shared)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["shared"] = self.shared
    return json
  }
}

public struct Test_Entry_Baz {
  let shared: String
}
public extension Test_Entry_Baz {
  static func decode(json: Any) throws -> Test_Entry_Baz {
    let json = try decode_value(json as? [String: Any])
    guard let f_shared = json["shared"] else {
      throw SerializationError.missing("shared")
    }

    let shared = try decode_name(unbox(f_shared, as: String.self), name: "shared")
    return Test_Entry_Baz(shared: shared)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["shared"] = self.shared
    return json
  }
}
