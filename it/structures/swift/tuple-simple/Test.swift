public struct Test_Entry {
  let tuple1: Test_Tuple1?
  let tuple2: Test_Tuple2?
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

// Complex object.
public struct Test_Other {
  let a: String
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
