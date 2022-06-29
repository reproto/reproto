enum SerializationError: Error {
  case missing(String)
  case invalid(String)
  case bad_value
}

func decode_name<T>(_ unbox: T?, name string: String) throws -> T {
  guard let value = unbox else {
    throw SerializationError.invalid(string)
  }

  return value
}

func decode_value<T>(_ value: T?) throws -> T {
  guard let value = value else {
    throw SerializationError.bad_value
  }

  return value
}

func unbox(_ value: Any, as type: Int.Type) -> Int? {
  switch value {
  case let n as UInt:
    return Int(exactly: n)
  case let n as Int32:
    return Int(exactly: n)
  case let n as Int64:
    return Int(exactly: n)
  case let n as UInt32:
    return Int(exactly: n)
  case let n as UInt64:
    return Int(exactly: n)
  case let n as Float:
    return Int(n)
  case let n as Double:
    return Int(n)
  default:
    return value as? Int
  }
}

func unbox(_ value: Any, as type: UInt.Type) -> UInt? {
  switch value {
  case let n as Int:
    return UInt(exactly: n)
  case let n as Int32:
    return UInt(exactly: n)
  case let n as Int64:
    return UInt(exactly: n)
  case let n as UInt32:
    return UInt(exactly: n)
  case let n as UInt64:
    return UInt(exactly: n)
  case let n as Float:
    return UInt(n)
  case let n as Double:
    return UInt(n)
  default:
    return value as? UInt
  }
}

func unbox(_ value: Any, as type: Int32.Type) -> Int32? {
  switch value {
  case let n as Int:
    return Int32(exactly: n)
  case let n as UInt:
    return Int32(exactly: n)
  case let n as Int64:
    return Int32(exactly: n)
  case let n as UInt32:
    return Int32(exactly: n)
  case let n as UInt64:
    return Int32(exactly: n)
  case let n as Float:
    return Int32(n)
  case let n as Double:
    return Int32(n)
  default:
    return value as? Int32
  }
}

func unbox(_ value: Any, as type: Int64.Type) -> Int64? {
  switch value {
  case let n as Int:
    return Int64(exactly: n)
  case let n as UInt:
    return Int64(exactly: n)
  case let n as Int32:
    return Int64(exactly: n)
  case let n as UInt32:
    return Int64(exactly: n)
  case let n as UInt64:
    return Int64(exactly: n)
  case let n as Float:
    return Int64(n)
  case let n as Double:
    return Int64(n)
  default:
    return value as? Int64
  }
}

func unbox(_ value: Any, as type: UInt32.Type) -> UInt32? {
  switch value {
  case let n as Int:
    return UInt32(exactly: n)
  case let n as UInt:
    return UInt32(exactly: n)
  case let n as Int32:
    return UInt32(exactly: n)
  case let n as Int64:
    return UInt32(exactly: n)
  case let n as UInt64:
    return UInt32(exactly: n)
  case let n as Float:
    return UInt32(n)
  case let n as Double:
    return UInt32(n)
  default:
    return value as? UInt32
  }
}

func unbox(_ value: Any, as type: UInt64.Type) -> UInt64? {
  switch value {
  case let n as Int:
    return UInt64(exactly: n)
  case let n as UInt:
    return UInt64(exactly: n)
  case let n as Int32:
    return UInt64(exactly: n)
  case let n as Int64:
    return UInt64(exactly: n)
  case let n as UInt32:
    return UInt64(exactly: n)
  case let n as Float:
    return UInt64(n)
  case let n as Double:
    return UInt64(n)
  default:
    return value as? UInt64
  }
}

func unbox(_ value: Any, as type: Float.Type) -> Float? {
  switch value {
  case let n as Int:
    return Float(exactly: n)
  case let n as UInt:
    return Float(exactly: n)
  case let n as Int32:
    return Float(exactly: n)
  case let n as Int64:
    return Float(exactly: n)
  case let n as UInt32:
    return Float(exactly: n)
  case let n as UInt64:
    return Float(exactly: n)
  case let n as Double:
    return Float(n)
  default:
    return value as? Float
  }
}

func unbox(_ value: Any, as type: Double.Type) -> Double? {
  switch value {
  case let n as Int:
    return Double(exactly: n)
  case let n as UInt:
    return Double(exactly: n)
  case let n as Int32:
    return Double(exactly: n)
  case let n as Int64:
    return Double(exactly: n)
  case let n as UInt32:
    return Double(exactly: n)
  case let n as UInt64:
    return Double(exactly: n)
  case let n as Float:
    return Double(n)
  default:
    return value as? Double
  }
}

func unbox(_ value: Any, as type: String.Type) -> String? {
  return value as? String
}

func unbox(_ value: Any, as type: Bool.Type) -> Bool? {
  return value as? Bool
}

func decode_array<T>(_ value: Any, name: String, inner: (Any) throws -> T) throws -> [T] {
  let array = try decode_name(value as? [Any], name: name)
  var out = [T]()

  for item in array {
    out.append(try inner(item))
  }

  return out
}

func encode_array<T>(_ array: [T], name: String, inner: (T) throws -> Any) throws -> [Any] {
  var out = [Any]()

  for item in array {
    out.append(try inner(item))
  }

  return out
}

func decode_map<T>(_ map: Any, name: String, value: (Any) throws -> T) throws -> [String: T] {
  let map = try decode_name(map as? [String: Any], name: name)
  var out = [String: T]()

  for (k, v) in map {
    out[k] = try value(v)
  }

  return out
}

func encode_map<T>(_ map: [String: T], name: String, value: (T) throws -> Any) throws -> [String: Any] {
  var out = [String: Any]()

  for (k, v) in map {
    out[k] = try value(v)
  }

  return out
}
