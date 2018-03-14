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

public struct Test_RootType: Codable {
}

public extension Test_RootType {
  static func decode(json: Any) throws -> Test_RootType {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootInterface {
  case Foo(Test_RootInterface_Foo)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_RootInterface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "Foo":
      self = try .Foo(Test_RootInterface_Foo(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootInterface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .Foo(let d):
      try values.encode("Foo", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_RootInterface {
  static func decode(json: Any) throws -> Test_RootInterface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "Foo":
        let v = try Test_RootInterface_Foo.decode(json: json)
        return Test_RootInterface.Foo(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .Foo(let s):
        var json = try s.encode()
        json["type"] = "Foo"
        return json
    }
  }
}

public struct Test_RootInterface_Foo: Codable {
}
public extension Test_RootInterface_Foo {
  static func decode(json: Any) throws -> Test_RootInterface_Foo {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootEnum {
  case Foo
}

public extension Test_RootEnum {
  static func decode(json: Any) throws -> Test_RootEnum {
    let json = try decode_value(json as? String)

    switch json {
      case "Foo":
        return Test_RootEnum.Foo
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Foo:
        return "Foo"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_RootEnum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Foo":
      self = .Foo
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootEnum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Foo:
      try value.encode("Foo")
    }
  }
}

public struct Test_RootTuple {
}

public extension Test_RootTuple {
  static func decode(json: Any) throws -> Test_RootTuple {
    let json = try decode_value(json as? [Any])
    return Test_RootTuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_RootTuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_RootTuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}

public struct Test_RootType_NestedType: Codable {
}

public extension Test_RootType_NestedType {
  static func decode(json: Any) throws -> Test_RootType_NestedType {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType_NestedType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootType_NestedInterface {
  case Foo(Test_RootType_NestedInterface_Foo)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_RootType_NestedInterface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "Foo":
      self = try .Foo(Test_RootType_NestedInterface_Foo(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootType_NestedInterface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .Foo(let d):
      try values.encode("Foo", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_RootType_NestedInterface {
  static func decode(json: Any) throws -> Test_RootType_NestedInterface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "Foo":
        let v = try Test_RootType_NestedInterface_Foo.decode(json: json)
        return Test_RootType_NestedInterface.Foo(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .Foo(let s):
        var json = try s.encode()
        json["type"] = "Foo"
        return json
    }
  }
}

public struct Test_RootType_NestedInterface_Foo: Codable {
}
public extension Test_RootType_NestedInterface_Foo {
  static func decode(json: Any) throws -> Test_RootType_NestedInterface_Foo {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType_NestedInterface_Foo()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootType_NestedEnum {
  case Foo
}

public extension Test_RootType_NestedEnum {
  static func decode(json: Any) throws -> Test_RootType_NestedEnum {
    let json = try decode_value(json as? String)

    switch json {
      case "Foo":
        return Test_RootType_NestedEnum.Foo
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Foo:
        return "Foo"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_RootType_NestedEnum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Foo":
      self = .Foo
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootType_NestedEnum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Foo:
      try value.encode("Foo")
    }
  }
}

public struct Test_RootType_NestedTuple {
}

public extension Test_RootType_NestedTuple {
  static func decode(json: Any) throws -> Test_RootType_NestedTuple {
    let json = try decode_value(json as? [Any])
    return Test_RootType_NestedTuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_RootType_NestedTuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_RootType_NestedTuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}

public struct Test_RootInterface_Foo_NestedType: Codable {
}

public extension Test_RootInterface_Foo_NestedType {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedType {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo_NestedType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootInterface_Foo_NestedInterface {
  case NestedFoo(Test_RootInterface_Foo_NestedInterface_NestedFoo)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_RootInterface_Foo_NestedInterface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "NestedFoo":
      self = try .NestedFoo(Test_RootInterface_Foo_NestedInterface_NestedFoo(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootInterface_Foo_NestedInterface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .NestedFoo(let d):
      try values.encode("NestedFoo", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_RootInterface_Foo_NestedInterface {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedInterface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "NestedFoo":
        let v = try Test_RootInterface_Foo_NestedInterface_NestedFoo.decode(json: json)
        return Test_RootInterface_Foo_NestedInterface.NestedFoo(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .NestedFoo(let s):
        var json = try s.encode()
        json["type"] = "NestedFoo"
        return json
    }
  }
}

public struct Test_RootInterface_Foo_NestedInterface_NestedFoo: Codable {
}
public extension Test_RootInterface_Foo_NestedInterface_NestedFoo {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedInterface_NestedFoo {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo_NestedInterface_NestedFoo()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootInterface_Foo_NestedEnum {
  case Foo
}

public extension Test_RootInterface_Foo_NestedEnum {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedEnum {
    let json = try decode_value(json as? String)

    switch json {
      case "Foo":
        return Test_RootInterface_Foo_NestedEnum.Foo
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Foo:
        return "Foo"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_RootInterface_Foo_NestedEnum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Foo":
      self = .Foo
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootInterface_Foo_NestedEnum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Foo:
      try value.encode("Foo")
    }
  }
}

public struct Test_RootInterface_Foo_NestedTuple {
}

public extension Test_RootInterface_Foo_NestedTuple {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedTuple {
    let json = try decode_value(json as? [Any])
    return Test_RootInterface_Foo_NestedTuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_RootInterface_Foo_NestedTuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_RootInterface_Foo_NestedTuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}

public struct Test_RootTuple_NestedType: Codable {
}

public extension Test_RootTuple_NestedType {
  static func decode(json: Any) throws -> Test_RootTuple_NestedType {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootTuple_NestedType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootTuple_NestedInterface {
  case Foo(Test_RootTuple_NestedInterface_Foo)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_RootTuple_NestedInterface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "Foo":
      self = try .Foo(Test_RootTuple_NestedInterface_Foo(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootTuple_NestedInterface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .Foo(let d):
      try values.encode("Foo", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_RootTuple_NestedInterface {
  static func decode(json: Any) throws -> Test_RootTuple_NestedInterface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "Foo":
        let v = try Test_RootTuple_NestedInterface_Foo.decode(json: json)
        return Test_RootTuple_NestedInterface.Foo(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .Foo(let s):
        var json = try s.encode()
        json["type"] = "Foo"
        return json
    }
  }
}

public struct Test_RootTuple_NestedInterface_Foo: Codable {
}
public extension Test_RootTuple_NestedInterface_Foo {
  static func decode(json: Any) throws -> Test_RootTuple_NestedInterface_Foo {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootTuple_NestedInterface_Foo()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootTuple_NestedEnum {
  case Foo
}

public extension Test_RootTuple_NestedEnum {
  static func decode(json: Any) throws -> Test_RootTuple_NestedEnum {
    let json = try decode_value(json as? String)

    switch json {
      case "Foo":
        return Test_RootTuple_NestedEnum.Foo
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Foo:
        return "Foo"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_RootTuple_NestedEnum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Foo":
      self = .Foo
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootTuple_NestedEnum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Foo:
      try value.encode("Foo")
    }
  }
}

public struct Test_RootTuple_NestedTuple {
}

public extension Test_RootTuple_NestedTuple {
  static func decode(json: Any) throws -> Test_RootTuple_NestedTuple {
    let json = try decode_value(json as? [Any])
    return Test_RootTuple_NestedTuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_RootTuple_NestedTuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_RootTuple_NestedTuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}

public struct Test_RootService_NestedType: Codable {
}

public extension Test_RootService_NestedType {
  static func decode(json: Any) throws -> Test_RootService_NestedType {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootService_NestedType()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootService_NestedInterface {
  case Foo(Test_RootService_NestedInterface_Foo)
  enum CodingKeys: String, CodingKey {
    case tag = "type"
  }
}

extension Test_RootService_NestedInterface: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "Foo":
      self = try .Foo(Test_RootService_NestedInterface_Foo(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootService_NestedInterface: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .Foo(let d):
      try values.encode("Foo", forKey: .tag)
      try d.encode(to: encoder)
    }
  }
}

public extension Test_RootService_NestedInterface {
  static func decode(json: Any) throws -> Test_RootService_NestedInterface {
    let json = try decode_value(json as? [String: Any])

    let type = try decode_name(json["type"] as? String, name: "type")

    switch type {
      case "Foo":
        let v = try Test_RootService_NestedInterface_Foo.decode(json: json)
        return Test_RootService_NestedInterface.Foo(v)
      default:
        throw SerializationError.invalid(type)
    }
  }

  func encode() throws -> [String: Any] {
    switch self {
      case .Foo(let s):
        var json = try s.encode()
        json["type"] = "Foo"
        return json
    }
  }
}

public struct Test_RootService_NestedInterface_Foo: Codable {
}
public extension Test_RootService_NestedInterface_Foo {
  static func decode(json: Any) throws -> Test_RootService_NestedInterface_Foo {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootService_NestedInterface_Foo()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public enum Test_RootService_NestedEnum {
  case Foo
}

public extension Test_RootService_NestedEnum {
  static func decode(json: Any) throws -> Test_RootService_NestedEnum {
    let json = try decode_value(json as? String)

    switch json {
      case "Foo":
        return Test_RootService_NestedEnum.Foo
      default:
        throw SerializationError.bad_value()
    }
  }

  func encode() throws -> String {
    switch self {
      case .Foo:
        return "Foo"
      default:
        throw SerializationError.bad_value()
    }
  }
}

extension Test_RootService_NestedEnum: Decodable {
  public init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer()

    switch try value.decode(String.self) {
    case "Foo":
      self = .Foo
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "enum variant")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_RootService_NestedEnum: Encodable {
  public func encode(to encoder: Encoder) throws {
    var value = encoder.singleValueContainer()

    switch self {
    case .Foo:
      try value.encode("Foo")
    }
  }
}

public struct Test_RootService_NestedTuple {
}

public extension Test_RootService_NestedTuple {
  static func decode(json: Any) throws -> Test_RootService_NestedTuple {
    let json = try decode_value(json as? [Any])
    return Test_RootService_NestedTuple()
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    return json
  }
}

extension Test_RootService_NestedTuple: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

  }
}

extension Test_RootService_NestedTuple: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

  }
}

public struct Test_RootType_NestedInterface_Foo_Nested: Codable {
}

public extension Test_RootType_NestedInterface_Foo_Nested {
  static func decode(json: Any) throws -> Test_RootType_NestedInterface_Foo_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType_NestedInterface_Foo_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootType_NestedTuple_Nested: Codable {
}

public extension Test_RootType_NestedTuple_Nested {
  static func decode(json: Any) throws -> Test_RootType_NestedTuple_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType_NestedTuple_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootType_NestedService_Nested: Codable {
}

public extension Test_RootType_NestedService_Nested {
  static func decode(json: Any) throws -> Test_RootType_NestedService_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootType_NestedService_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootInterface_Foo_NestedInterface_NestedFoo_Nested: Codable {
}

public extension Test_RootInterface_Foo_NestedInterface_NestedFoo_Nested {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedInterface_NestedFoo_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo_NestedInterface_NestedFoo_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootInterface_Foo_NestedTuple_Nested: Codable {
}

public extension Test_RootInterface_Foo_NestedTuple_Nested {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedTuple_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo_NestedTuple_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootInterface_Foo_NestedService_Nested: Codable {
}

public extension Test_RootInterface_Foo_NestedService_Nested {
  static func decode(json: Any) throws -> Test_RootInterface_Foo_NestedService_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootInterface_Foo_NestedService_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootTuple_NestedInterface_Foo_Nested: Codable {
}

public extension Test_RootTuple_NestedInterface_Foo_Nested {
  static func decode(json: Any) throws -> Test_RootTuple_NestedInterface_Foo_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootTuple_NestedInterface_Foo_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootTuple_NestedTuple_Nested: Codable {
}

public extension Test_RootTuple_NestedTuple_Nested {
  static func decode(json: Any) throws -> Test_RootTuple_NestedTuple_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootTuple_NestedTuple_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootTuple_NestedService_Nested: Codable {
}

public extension Test_RootTuple_NestedService_Nested {
  static func decode(json: Any) throws -> Test_RootTuple_NestedService_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootTuple_NestedService_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootService_NestedInterface_Foo_Nested: Codable {
}

public extension Test_RootService_NestedInterface_Foo_Nested {
  static func decode(json: Any) throws -> Test_RootService_NestedInterface_Foo_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootService_NestedInterface_Foo_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootService_NestedTuple_Nested: Codable {
}

public extension Test_RootService_NestedTuple_Nested {
  static func decode(json: Any) throws -> Test_RootService_NestedTuple_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootService_NestedTuple_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}

public struct Test_RootService_NestedService_Nested: Codable {
}

public extension Test_RootService_NestedService_Nested {
  static func decode(json: Any) throws -> Test_RootService_NestedService_Nested {
    let _ = try decode_value(json as? [String: Any])

    return Test_RootService_NestedService_Nested()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}
