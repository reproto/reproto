public struct Test_Entry: Codable {
  let lower_camel: LowerCamel_Value?
  let lower_snake: LowerSnake_Value?
  let upper_camel: UpperCamel_Value?
  let upper_snake: UpperSnake_Value?

  enum CodingKeys: String, CodingKey {
    case lower_camel = "lower_camel"
    case lower_snake = "lower_snake"
    case upper_camel = "upper_camel"
    case upper_snake = "upper_snake"
  }
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var lower_camel: LowerCamel_Value? = Optional.none

    if let value = json["lower_camel"] {
      lower_camel = Optional.some(try LowerCamel_Value.decode(json: value))
    }

    var lower_snake: LowerSnake_Value? = Optional.none

    if let value = json["lower_snake"] {
      lower_snake = Optional.some(try LowerSnake_Value.decode(json: value))
    }

    var upper_camel: UpperCamel_Value? = Optional.none

    if let value = json["upper_camel"] {
      upper_camel = Optional.some(try UpperCamel_Value.decode(json: value))
    }

    var upper_snake: UpperSnake_Value? = Optional.none

    if let value = json["upper_snake"] {
      upper_snake = Optional.some(try UpperSnake_Value.decode(json: value))
    }

    return Test_Entry(lower_camel: lower_camel, lower_snake: lower_snake, upper_camel: upper_camel, upper_snake: upper_snake)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.lower_camel {
      json["lower_camel"] = try value.encode()
    }
    if let value = self.lower_snake {
      json["lower_snake"] = try value.encode()
    }
    if let value = self.upper_camel {
      json["upper_camel"] = try value.encode()
    }
    if let value = self.upper_snake {
      json["upper_snake"] = try value.encode()
    }

    return json
  }
}
