public struct LowerCamel_Value {
  let foo_bar: String
}

public extension LowerCamel_Value {
  static func decode(json: Any) throws -> LowerCamel_Value {
    let json = try decode_value(json as? [String: Any])

    guard let f_foo_bar = json["fooBar"] else {
      throw SerializationError.missing("fooBar")
    }

    let foo_bar = try decode_name(unbox(f_foo_bar, as: String.self), name: "fooBar")
    return LowerCamel_Value(foo_bar: foo_bar)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["fooBar"] = self.foo_bar

    return json
  }
}
