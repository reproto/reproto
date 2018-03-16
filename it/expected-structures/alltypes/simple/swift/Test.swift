import Foundation

public struct Test_Entry {
  let boolean_type: Bool?
  let string_type: String?
  let datetime_type: Date?
  let unsigned_32: UInt32?
  let unsigned_64: UInt64?
  let signed_32: Int32?
  let signed_64: Int64?
  let float_type: Float?
  let double_type: Double?
  let bytes_type: Data?
  let any_type: Any?
  let array_type: [Test_Entry]?
  let array_of_array_type: [[Test_Entry]]?
  let map_type: [String: Test_Entry]?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var boolean_type: Bool? = Optional.none

    if let value = json["boolean_type"] {
      boolean_type = Optional.some(try decode_name(unbox(value, as: Bool.self), name: "boolean_type"))
    }

    var string_type: String? = Optional.none

    if let value = json["string_type"] {
      string_type = Optional.some(try decode_name(unbox(value, as: String.self), name: "string_type"))
    }

    var datetime_type: Date? = Optional.none

    if let value = json["datetime_type"] {
      datetime_type = Optional.some(try decode_name(try decode_value(ISO8601DateFormatter().date(from: try decode_value(value as? String))), name: "datetime_type"))
    }

    var unsigned_32: UInt32? = Optional.none

    if let value = json["unsigned_32"] {
      unsigned_32 = Optional.some(try decode_name(unbox(value, as: UInt32.self), name: "unsigned_32"))
    }

    var unsigned_64: UInt64? = Optional.none

    if let value = json["unsigned_64"] {
      unsigned_64 = Optional.some(try decode_name(unbox(value, as: UInt64.self), name: "unsigned_64"))
    }

    var signed_32: Int32? = Optional.none

    if let value = json["signed_32"] {
      signed_32 = Optional.some(try decode_name(unbox(value, as: Int32.self), name: "signed_32"))
    }

    var signed_64: Int64? = Optional.none

    if let value = json["signed_64"] {
      signed_64 = Optional.some(try decode_name(unbox(value, as: Int64.self), name: "signed_64"))
    }

    var float_type: Float? = Optional.none

    if let value = json["float_type"] {
      float_type = Optional.some(try decode_name(unbox(value, as: Float.self), name: "float_type"))
    }

    var double_type: Double? = Optional.none

    if let value = json["double_type"] {
      double_type = Optional.some(try decode_name(unbox(value, as: Double.self), name: "double_type"))
    }

    var bytes_type: Data? = Optional.none

    if let value = json["bytes_type"] {
      bytes_type = Optional.some(try decode_name(Data(base64Encoded: try decode_value(value as? String)), name: "bytes_type"))
    }

    var any_type: Any? = Optional.none

    if let value = json["any_type"] {
      any_type = Optional.some(try decode_name(value, name: "any_type"))
    }

    var array_type: [Test_Entry]? = Optional.none

    if let value = json["array_type"] {
      array_type = Optional.some(try decode_array(value, name: "array_type", inner: { inner in try Test_Entry.decode(json: inner) }))
    }

    var array_of_array_type: [[Test_Entry]]? = Optional.none

    if let value = json["array_of_array_type"] {
      array_of_array_type = Optional.some(try decode_array(value, name: "array_of_array_type", inner: { inner in try decode_array(inner, name: "array_of_array_type", inner: { inner in try Test_Entry.decode(json: inner) }) }))
    }

    var map_type: [String: Test_Entry]? = Optional.none

    if let value = json["map_type"] {
      map_type = Optional.some(try decode_map(value, name: "map_type", value: { value in try Test_Entry.decode(json: value) }))
    }

    return Test_Entry(boolean_type: boolean_type, string_type: string_type, datetime_type: datetime_type, unsigned_32: unsigned_32, unsigned_64: unsigned_64, signed_32: signed_32, signed_64: signed_64, float_type: float_type, double_type: double_type, bytes_type: bytes_type, any_type: any_type, array_type: array_type, array_of_array_type: array_of_array_type, map_type: map_type)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.boolean_type {
      json["boolean_type"] = value
    }
    if let value = self.string_type {
      json["string_type"] = value
    }
    if let value = self.datetime_type {
      json["datetime_type"] = ISO8601DateFormatter().string(from: value)
    }
    if let value = self.unsigned_32 {
      json["unsigned_32"] = value
    }
    if let value = self.unsigned_64 {
      json["unsigned_64"] = value
    }
    if let value = self.signed_32 {
      json["signed_32"] = value
    }
    if let value = self.signed_64 {
      json["signed_64"] = value
    }
    if let value = self.float_type {
      json["float_type"] = value
    }
    if let value = self.double_type {
      json["double_type"] = value
    }
    if let value = self.bytes_type {
      json["bytes_type"] = value.base64EncodedString()
    }
    if let value = self.any_type {
      json["any_type"] = value
    }
    if let value = self.array_type {
      json["array_type"] = try encode_array(value, name: "array_type", inner: { inner in try inner.encode() })
    }
    if let value = self.array_of_array_type {
      json["array_of_array_type"] = try encode_array(value, name: "array_of_array_type", inner: { inner in try encode_array(inner, name: "array_of_array_type", inner: { inner in try inner.encode() }) })
    }
    if let value = self.map_type {
      json["map_type"] = try encode_map(value, name: "map_type", value: { value in try value.encode() })
    }

    return json
  }
}
