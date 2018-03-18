public struct Test_Entry {
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

public struct Test_Type {
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

public struct Test_Interface_SubType {
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
