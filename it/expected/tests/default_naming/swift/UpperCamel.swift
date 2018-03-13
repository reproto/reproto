public struct UpperCamel_Value {
  let foo_bar: String
}

public extension UpperCamel_Value {
  static func decode(json: Any) throws -> UpperCamel_Value {
    let json = try decode_value(json as? [String: Any])
    guard let f_foo_bar = json["FooBar"] else {
      throw SerializationError.missing("FooBar")
    }

    let foo_bar = try decode_name(unbox(f_foo_bar, as: String.self), name: "FooBar")
    return UpperCamel_Value(foo_bar: foo_bar)
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    json["FooBar"] = self.foo_bar
    return json
  }
}
