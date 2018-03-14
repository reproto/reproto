public struct True_Empty {
}

public extension True_Empty {
  static func decode(json: Any) throws -> True_Empty {
    let json = try decode_value(json as? [String: Any])

    return True_Empty()
  }
  func encode() throws -> [String: Any] {
    var json = [String: Any]()
    return json
  }
}
