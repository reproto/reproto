public struct Test_Entry: Codable {
  let tuple1: Test_Tuple1?
  let tuple2: Test_Tuple2?

  enum CodingKeys: String, CodingKey {
    case tuple1 = "tuple1"
    case tuple2 = "tuple2"
  }
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var tuple1: Test_Tuple1? = Optional.none

    if let value = json["tuple1"] {
      tuple1 = Optional.some(try Test_Tuple1.decode(json: value))
    }

    var tuple2: Test_Tuple2? = Optional.none

    if let value = json["tuple2"] {
      tuple2 = Optional.some(try Test_Tuple2.decode(json: value))
    }

    return Test_Entry(tuple1: tuple1, tuple2: tuple2)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.tuple1 {
      json["tuple1"] = try value.encode()
    }
    if let value = self.tuple2 {
      json["tuple2"] = try value.encode()
    }

    return json
  }
}

// Tuple containing primitive.
public struct Test_Tuple1 {
  let a: String
  let b: UInt64
}

public extension Test_Tuple1 {
  static func decode(json: Any) throws -> Test_Tuple1 {
    let json = try decode_value(json as? [Any])

    guard let f_a = Optional.some(json[0]) else {
      throw SerializationError.missing("[0]")
    }

    let a = try decode_name(unbox(f_a, as: String.self), name: "[0]")

    guard let f_b = Optional.some(json[1]) else {
      throw SerializationError.missing("[1]")
    }

    let b = try decode_name(unbox(f_b, as: UInt64.self), name: "[1]")
    return Test_Tuple1(a: a, b: b)
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    json.append(self.a)
    json.append(self.b)

    return json
  }
}

extension Test_Tuple1: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

    self.a = try values.decode(String.self)
    self.b = try values.decode(UInt64.self)
  }
}

extension Test_Tuple1: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

    try values.encode(self.a)
    try values.encode(self.b)
  }
}

// Tuple containing object.
public struct Test_Tuple2 {
  let a: String
  let b: Test_Other
}

public extension Test_Tuple2 {
  static func decode(json: Any) throws -> Test_Tuple2 {
    let json = try decode_value(json as? [Any])

    guard let f_a = Optional.some(json[0]) else {
      throw SerializationError.missing("[0]")
    }

    let a = try decode_name(unbox(f_a, as: String.self), name: "[0]")

    guard let f_b = Optional.some(json[1]) else {
      throw SerializationError.missing("[1]")
    }

    let b = try Test_Other.decode(json: f_b)
    return Test_Tuple2(a: a, b: b)
  }

  func encode() throws -> [Any] {
    var json = [Any]()

    json.append(self.a)
    json.append(try self.b.encode())

    return json
  }
}

extension Test_Tuple2: Decodable {
  public init(from decoder: Decoder) throws {
    var values = try decoder.unkeyedContainer()

    self.a = try values.decode(String.self)
    self.b = try values.decode(Test_Other.self)
  }
}

extension Test_Tuple2: Encodable {
  public func encode(to encoder: Encoder) throws {
    var values = encoder.unkeyedContainer()

    try values.encode(self.a)
    try values.encode(self.b)
  }
}

// Complex object.
public struct Test_Other: Codable {
  let a: String

  enum CodingKeys: String, CodingKey {
    case a = "a"
  }
}

public extension Test_Other {
  static func decode(json: Any) throws -> Test_Other {
    let json = try decode_value(json as? [String: Any])

    guard let f_a = json["a"] else {
      throw SerializationError.missing("a")
    }

    let a = try decode_name(unbox(f_a, as: String.self), name: "a")

    return Test_Other(a: a)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["a"] = self.a

    return json
  }
}
