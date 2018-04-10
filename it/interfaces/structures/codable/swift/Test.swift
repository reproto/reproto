public struct Test_Entry: Codable {
  let tagged: Test_Tagged?
  let required_fields: Test_RequiredFields?

  enum CodingKeys: String, CodingKey {
    case tagged = "tagged"
    case required_fields = "required_fields"
  }
}

public enum Test_Tagged {
  case A(Test_Tagged_A)
  case B(Test_Tagged_B)
  case Bar(Test_Tagged_Bar)
  case Baz(Test_Tagged_Baz)

  enum CodingKeys: String, CodingKey {
    case tag = "@type"
  }
}

extension Test_Tagged: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "foo":
      self = try .A(Test_Tagged_A(from: decoder))
    case "b":
      self = try .B(Test_Tagged_B(from: decoder))
    case "Bar":
      self = try .Bar(Test_Tagged_Bar(from: decoder))
    case "Baz":
      self = try .Baz(Test_Tagged_Baz(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "@type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_Tagged: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .A(let d):
      try values.encode("foo", forKey: .tag)
      try d.encode(to: encoder)
    case .B(let d):
      try values.encode("b", forKey: .tag)
      try d.encode(to: encoder)
    case .Bar(let d):
      try values.encode("Bar", forKey: .tag)
      try d.encode(to: encoder)
    case .Baz(let d):
      try values.encode("Baz", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public struct Test_Tagged_A: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
}

public struct Test_Tagged_B: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
}

public struct Test_Tagged_Bar: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
}

public struct Test_Tagged_Baz: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
}

public enum Test_RequiredFields {
  case A(Test_RequiredFields_A)
  case B(Test_RequiredFields_B)
  case C(Test_RequiredFields_C)

  enum AKeys: String, CodingKey {
    case a = "a"
    case b = "b"
  }

  enum BKeys: String, CodingKey {
    case a = "a"
    case _k0 = "b"
  }

  enum CKeys: String, CodingKey {
    case b = "b"
    case _k0 = "a"
  }
}

extension Test_RequiredFields: Decodable {
  public init(from decoder: Decoder) throws {
    if Set(try decoder.container(keyedBy: AKeys.self).allKeys) == Set([AKeys.a, AKeys.b]) {
      self = try .A(Test_RequiredFields_A(from: decoder))
      return
    }

    if Set(try decoder.container(keyedBy: BKeys.self).allKeys) == Set([BKeys.a]) {
      self = try .B(Test_RequiredFields_B(from: decoder))
      return
    }

    if Set(try decoder.container(keyedBy: CKeys.self).allKeys) == Set([CKeys.b]) {
      self = try .C(Test_RequiredFields_C(from: decoder))
      return
    }

    let context = DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "no legal field combination")
    throw DecodingError.dataCorrupted(context)
  }
}

extension Test_RequiredFields: Encodable {
  public func encode(to encoder: Encoder) throws {
    switch self {
    case .A(let d):
      try d.encode(to: encoder)
    case .B(let d):
      try d.encode(to: encoder)
    case .C(let d):
      try d.encode(to: encoder)
    }
  }
}

// Special case: fields shared with other sub-types.
// NOTE: due to rust support through untagged, the types are matched in-order.
public struct Test_RequiredFields_A: Codable {
  let shared: String
  let shared_ignore: String?
  let a: String
  let b: String
  let ignore: String?

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
    case shared_ignore = "shared_ignore"
    case a = "a"
    case b = "b"
    case ignore = "ignore"
  }
}

public struct Test_RequiredFields_B: Codable {
  let shared: String
  let shared_ignore: String?
  let a: String
  let ignore: String?

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
    case shared_ignore = "shared_ignore"
    case a = "a"
    case ignore = "ignore"
  }
}

public struct Test_RequiredFields_C: Codable {
  let shared: String
  let shared_ignore: String?
  let b: String
  let ignore: String?

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
    case shared_ignore = "shared_ignore"
    case b = "b"
    case ignore = "ignore"
  }
}
