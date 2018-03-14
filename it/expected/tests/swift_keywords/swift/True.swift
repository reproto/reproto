public struct True_Empty: Codable {
}

public extension True_Empty {
  static func decode(json: Any) throws -> True_Empty {
    let _ = try decode_value(json as? [String: Any])

    return True_Empty()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}
