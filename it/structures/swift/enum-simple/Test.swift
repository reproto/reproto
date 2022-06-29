public struct Test_Entry {
  let explicit: Test_EnumExplicit?
  let implicit: Test_EnumImplicit?
  let enum_u32: Test_EnumU32?
  let enum_u64: Test_EnumU64?
  let enum_i32: Test_EnumI32?
  let enum_i64: Test_EnumI64?
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

    var enum_u32: Test_EnumU32? = Optional.none

    if let value = json["enum_u32"] {
      enum_u32 = Optional.some(try Test_EnumU32.decode(json: value))
    }

    var enum_u64: Test_EnumU64? = Optional.none

    if let value = json["enum_u64"] {
      enum_u64 = Optional.some(try Test_EnumU64.decode(json: value))
    }

    var enum_i32: Test_EnumI32? = Optional.none

    if let value = json["enum_i32"] {
      enum_i32 = Optional.some(try Test_EnumI32.decode(json: value))
    }

    var enum_i64: Test_EnumI64? = Optional.none

    if let value = json["enum_i64"] {
      enum_i64 = Optional.some(try Test_EnumI64.decode(json: value))
    }
    return Test_Entry(explicit: explicit, implicit: implicit, enum_u32: enum_u32, enum_u64: enum_u64, enum_i32: enum_i32, enum_i64: enum_i64)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.explicit {
      json["explicit"] = try value.encode()
    }

    if let value = self.implicit {
      json["implicit"] = try value.encode()
    }

    if let value = self.enum_u32 {
      json["enum_u32"] = try value.encode()
    }

    if let value = self.enum_u64 {
      json["enum_u64"] = try value.encode()
    }

    if let value = self.enum_i32 {
      json["enum_i32"] = try value.encode()
    }

    if let value = self.enum_i64 {
      json["enum_i64"] = try value.encode()
    }

    return json
  }
}

public enum Test_EnumExplicit {
  case A
  case B
}

public extension Test_EnumExplicit {
  static func decode(json: Any) throws -> Test_EnumExplicit {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: String.self))

    switch value {
    case "foo":
      return Test_EnumExplicit.A
    case "bar":
      return Test_EnumExplicit.B
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> String {
    switch self {
    case .A:
      return "foo"
    case .B:
      return "bar"
    }
  }
}

public enum Test_EnumImplicit {
  case A
  case B
}

public extension Test_EnumImplicit {
  static func decode(json: Any) throws -> Test_EnumImplicit {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: String.self))

    switch value {
    case "A":
      return Test_EnumImplicit.A
    case "B":
      return Test_EnumImplicit.B
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> String {
    switch self {
    case .A:
      return "A"
    case .B:
      return "B"
    }
  }
}

public enum Test_EnumLongNames {
  case FooBar
  case Baz
}

public extension Test_EnumLongNames {
  static func decode(json: Any) throws -> Test_EnumLongNames {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: String.self))

    switch value {
    case "FooBar":
      return Test_EnumLongNames.FooBar
    case "Baz":
      return Test_EnumLongNames.Baz
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> String {
    switch self {
    case .FooBar:
      return "FooBar"
    case .Baz:
      return "Baz"
    }
  }
}

public enum Test_EnumU32 {
  case Min
  case Max
}

public extension Test_EnumU32 {
  static func decode(json: Any) throws -> Test_EnumU32 {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: UInt32.self))

    switch value {
    case 0:
      return Test_EnumU32.Min
    case 2147483647:
      return Test_EnumU32.Max
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> UInt32 {
    switch self {
    case .Min:
      return 0
    case .Max:
      return 2147483647
    }
  }
}

public enum Test_EnumU64 {
  case Min
  case Max
}

public extension Test_EnumU64 {
  static func decode(json: Any) throws -> Test_EnumU64 {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: UInt64.self))

    switch value {
    case 0:
      return Test_EnumU64.Min
    case 9007199254740991:
      return Test_EnumU64.Max
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> UInt64 {
    switch self {
    case .Min:
      return 0
    case .Max:
      return 9007199254740991
    }
  }
}

public enum Test_EnumI32 {
  case Min
  case NegativeOne
  case Zero
  case Max
}

public extension Test_EnumI32 {
  static func decode(json: Any) throws -> Test_EnumI32 {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: Int32.self))

    switch value {
    case -2147483648:
      return Test_EnumI32.Min
    case -1:
      return Test_EnumI32.NegativeOne
    case 0:
      return Test_EnumI32.Zero
    case 2147483647:
      return Test_EnumI32.Max
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> Int32 {
    switch self {
    case .Min:
      return -2147483648
    case .NegativeOne:
      return -1
    case .Zero:
      return 0
    case .Max:
      return 2147483647
    }
  }
}

public enum Test_EnumI64 {
  case Min
  case NegativeOne
  case Zero
  case Max
}

public extension Test_EnumI64 {
  static func decode(json: Any) throws -> Test_EnumI64 {
    let json = try decode_value(json)
    let value = try decode_value(unbox(json, as: Int64.self))

    switch value {
    case -9007199254740991:
      return Test_EnumI64.Min
    case -1:
      return Test_EnumI64.NegativeOne
    case 0:
      return Test_EnumI64.Zero
    case 9007199254740991:
      return Test_EnumI64.Max
    default:
      throw SerializationError.bad_value
    }
  }

  func encode() throws -> Int64 {
    switch self {
    case .Min:
      return -9007199254740991
    case .NegativeOne:
      return -1
    case .Zero:
      return 0
    case .Max:
      return 9007199254740991
    }
  }
}
