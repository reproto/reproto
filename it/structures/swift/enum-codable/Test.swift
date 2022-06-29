public struct Test_Entry: Codable {
  let explicit: Test_EnumExplicit?
  let implicit: Test_EnumImplicit?
  let enum_u32: Test_EnumU32?
  let enum_u64: Test_EnumU64?
  let enum_i32: Test_EnumI32?
  let enum_i64: Test_EnumI64?

  enum CodingKeys: String, CodingKey {
    case explicit = "explicit"
    case implicit = "implicit"
    case enum_u32 = "enum_u32"
    case enum_u64 = "enum_u64"
    case enum_i32 = "enum_i32"
    case enum_i64 = "enum_i64"
  }
}

public enum Test_EnumExplicit {
  case A
  case B
}

extension Test_EnumExplicit: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "foo":
      self = .A
    case "bar":
      self = .B
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumExplicit: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .A:
      try value.encode("foo")
    case .B:
      try value.encode("bar")
    }
  }
}

public enum Test_EnumImplicit {
  case A
  case B
}

extension Test_EnumImplicit: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "A":
      self = .A
    case "B":
      self = .B
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumImplicit: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .A:
      try value.encode("A")
    case .B:
      try value.encode("B")
    }
  }
}

public enum Test_EnumLongNames {
  case FooBar
  case Baz
}

extension Test_EnumLongNames: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "FooBar":
      self = .FooBar
    case "Baz":
      self = .Baz
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumLongNames: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .FooBar:
      try value.encode("FooBar")
    case .Baz:
      try value.encode("Baz")
    }
  }
}

public enum Test_EnumU32 {
  case Min
  case Max
}

extension Test_EnumU32: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(UInt32.self) {
    case 0:
      self = .Min
    case 2147483647:
      self = .Max
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumU32: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Min:
      try value.encode(0)
    case .Max:
      try value.encode(2147483647)
    }
  }
}

public enum Test_EnumU64 {
  case Min
  case Max
}

extension Test_EnumU64: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(UInt64.self) {
    case 0:
      self = .Min
    case 9007199254740991:
      self = .Max
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumU64: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Min:
      try value.encode(0)
    case .Max:
      try value.encode(9007199254740991)
    }
  }
}

public enum Test_EnumI32 {
  case Min
  case NegativeOne
  case Zero
  case Max
}

extension Test_EnumI32: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(Int32.self) {
    case -2147483648:
      self = .Min
    case -1:
      self = .NegativeOne
    case 0:
      self = .Zero
    case 2147483647:
      self = .Max
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumI32: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Min:
      try value.encode(-2147483648)
    case .NegativeOne:
      try value.encode(-1)
    case .Zero:
      try value.encode(0)
    case .Max:
      try value.encode(2147483647)
    }
  }
}

public enum Test_EnumI64 {
  case Min
  case NegativeOne
  case Zero
  case Max
}

extension Test_EnumI64: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(Int64.self) {
    case -9007199254740991:
      self = .Min
    case -1:
      self = .NegativeOne
    case 0:
      self = .Zero
    case 9007199254740991:
      self = .Max
    default:
      let context = DecodingError.Context(
        codingPath: decoder.codingPath,
        debugDescription: "enum variant"
      )

      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_EnumI64: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Min:
      try value.encode(-9007199254740991)
    case .NegativeOne:
      try value.encode(-1)
    case .Zero:
      try value.encode(0)
    case .Max:
      try value.encode(9007199254740991)
    }
  }
}
