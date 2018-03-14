public struct Test_Entry: Codable {
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let _ = try decode_value(json as? [String: Any])

    return Test_Entry()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_Type: Codable {
}

public extension Test_Type {
  static func decode(json: Any) throws -> Test_Type {
    let _ = try decode_value(json as? [String: Any])

    return Test_Type()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_Interface {
  case SubType(Test_Interface_SubType)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_Interface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "SubType":
      self = try .SubType(Test_Interface_SubType(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_Interface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .SubType(let d):
      try values.encode("SubType", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_Interface {
  static func decode(json: Any) throws -> Test_Interface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "SubType":
        let v = try Test_Interface_SubType.decode(json: json)
        return Test_Interface.SubType(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .SubType(let s):
        var json = try s.encode()
        json["type"] = "SubType"
        return json
    }
  }
}

public struct Test_Interface_SubType: Codable {
}
public extension Test_Interface_SubType {
  static func decode(json: Any) throws -> Test_Interface_SubType {
    let _ = try decode_value(json as? [String: Any])

    return Test_Interface_SubType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_Enum {
  case Variant
}

public extension Test_Enum {
  static func decode(json: Any) throws -> Test_Enum {
    let json = try decode_value(json as? String)

    switch json {
      case "Variant":
        return Test_Enum.Variant
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Variant:
        return "Variant"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_Enum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Variant":
      self = .Variant
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_Enum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Variant:
      try value.encode("Variant")
    }
  }
}

public struct Test_Tuple {
}

public extension Test_Tuple {
  static func decode(json: Any) throws -> Test_Tuple {
    let json = try decode_value(json as? [Any])
    return Test_Tuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_Tuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_Tuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}
