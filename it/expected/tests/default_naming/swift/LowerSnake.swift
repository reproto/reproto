public struct LowerSnake_Value: Codable {
  let foo_bar: String

  enum CodingKeys: String, CodingKey {
    case foo_bar = "foo_bar"
  }
}

public extension LowerSnake_Value {
  static func decode(json: Any) throws -> LowerSnake_Value {
    let json = try decode_value(json as? [String: Any])

    guard let f_foo_bar = json["foo_bar"] else {
      throw SerializationError.missing("foo_bar")
    }

    let foo_bar = try decode_name(unbox(f_foo_bar, as: String.self), name: "foo_bar")

    return LowerSnake_Value(foo_bar: foo_bar)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    json["foo_bar"] = self.foo_bar

    return json
  }
}
