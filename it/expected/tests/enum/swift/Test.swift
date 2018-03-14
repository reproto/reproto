public struct Test_Entry {
  let explicit: Test_EnumExplicit?
  let implicit: Test_EnumImplicit?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var explicit: Test_EnumExplicit? = Optional.none

    if let value = json["explicit"] {
      explicit = Optional.some(try Test_EnumExplicit.decode(json: value))
    }

    var implicit: Test_EnumImplicit? = Optional.none

    if let value = json["implicit"] {
      implicit = Optional.some(try Test_EnumImplicit.decode(json: value))
    }

    return Test_Entry(explicit: explicit, implicit: implicit)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    if let value = self.explicit {
      json["explicit"] = try value.encode()
    }
    if let value = self.implicit {
      json["implicit"] = try value.encode()
    }
    return json
  }
}

public enum Test_EnumExplicit {
  case A()
  case B()
}

public extension Test_EnumExplicit {
  static func decode(json: Any) throws -> Test_EnumExplicit {
    let json = try decode_value(json as? String)

    switch json {
      case "foo":
        return Test_EnumExplicit.A()
      case "bar":
        return Test_EnumExplicit.B()
      default:
        throw SerializationError.bad_value()
    }
  }
  func encode() throws -> String {
    switch self {
      case .A:
        return "foo"
      case .B:
        return "bar"
      default:
        throw SerializationError.bad_value()
    }
  }
}

public enum Test_EnumImplicit {
  case A()
  case B()
}

public extension Test_EnumImplicit {
  static func decode(json: Any) throws -> Test_EnumImplicit {
    let json = try decode_value(json as? String)

    switch json {
      case "A":
        return Test_EnumImplicit.A()
      case "B":
        return Test_EnumImplicit.B()
      default:
        throw SerializationError.bad_value()
    }
  }
  func encode() throws -> String {
    switch self {
      case .A:
        return "A"
      case .B:
        return "B"
      default:
        throw SerializationError.bad_value()
    }
  }
}

public enum Test_EnumLongNames {
  case FooBar()
  case Baz()
}

public extension Test_EnumLongNames {
  static func decode(json: Any) throws -> Test_EnumLongNames {
    let json = try decode_value(json as? String)

    switch json {
      case "FooBar":
        return Test_EnumLongNames.FooBar()
      case "Baz":
        return Test_EnumLongNames.Baz()
      default:
        throw SerializationError.bad_value()
    }
  }
  func encode() throws -> String {
    switch self {
      case .FooBar:
        return "FooBar"
      case .Baz:
        return "Baz"
      default:
        throw SerializationError.bad_value()
    }
  }
}
