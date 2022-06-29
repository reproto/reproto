public struct Test_Entry {}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let _ = try decode_value(json as? [String: Any])

    return Test_Entry()
  }

  func encode() throws -> [String: Any] {
    return [String: Any]()
  }
}
