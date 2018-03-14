public enum Test_Entry {
  case A(Test_Entry_A)
  case B(Test_Entry_B)
  case Bar(Test_Entry_Bar)
  case Baz(Test_Entry_Baz)
  enum CodingKeys: String, CodingKey {
    case tag = "@type"
  }
}

extension Test_Entry: Decodable {
  public init(from decoder: Decoder) throws {
    let values = try decoder.container(keyedBy: CodingKeys.self)

    switch try values.decode(String.self, forKey: .tag) {
    case "foo":
      self = try .A(Test_Entry_A(from: decoder))
    case "b":
      self = try .B(Test_Entry_B(from: decoder))
    case "Bar":
      self = try .Bar(Test_Entry_Bar(from: decoder))
    case "Baz":
      self = try .Baz(Test_Entry_Baz(from: decoder))
    default:
      let context = DecodingError.Context(codingPath: [], debugDescription: "@type")
      throw DecodingError.dataCorrupted(context)
    }
  }
}

extension Test_Entry: Encodable {
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

public struct Test_Entry_A: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
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

public struct Test_Entry_B: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
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

public struct Test_Entry_Bar: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
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

public struct Test_Entry_Baz: Codable {
  let shared: String

  enum CodingKeys: String, CodingKey {
    case shared = "shared"
  }
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
