public struct UpperSnake_Value {
  let foo_bar: String
}

public extension UpperSnake_Value {
  static func decode(json: Any) throws -> UpperSnake_Value {
    let json = try decode_value(json as? [String: Any])

    guard let f_foo_bar = json["FOO_BAR"] else {
      throw SerializationError.missing("FOO_BAR")
    }

    let foo_bar = try decode_name(unbox(f_foo_bar, as: String.self), name: "FOO_BAR")
    return UpperSnake_Value(foo_bar: foo_bar)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["FOO_BAR"] = self.foo_bar

    return json
  }
}
