
export class Entry {
  constructor(boolean_type, string_type, datetime_type, unsigned_32, unsigned_64, signed_32, signed_64, float_type, double_type, bytes_type, any_type, array_type, map_type) {
    this.boolean_type = boolean_type;
    this.string_type = string_type;
    this.datetime_type = datetime_type;
    this.unsigned_32 = unsigned_32;
    this.unsigned_64 = unsigned_64;
    this.signed_32 = signed_32;
    this.signed_64 = signed_64;
    this.float_type = float_type;
    this.double_type = double_type;
    this.bytes_type = bytes_type;
    this.any_type = any_type;
    this.array_type = array_type;
    this.map_type = map_type;
  }

  static decode(data) {
    let v_boolean_type = data["boolean_type"];

    if (v_boolean_type !== null && v_boolean_type !== undefined) {
      v_boolean_type = v_boolean_type;
    } else {
      v_boolean_type = null;
    }

    let v_string_type = data["string_type"];

    if (v_string_type !== null && v_string_type !== undefined) {
      v_string_type = v_string_type;
    } else {
      v_string_type = null;
    }

    let v_datetime_type = data["datetime_type"];

    if (v_datetime_type !== null && v_datetime_type !== undefined) {
      v_datetime_type = v_datetime_type;
    } else {
      v_datetime_type = null;
    }

    let v_unsigned_32 = data["unsigned_32"];

    if (v_unsigned_32 !== null && v_unsigned_32 !== undefined) {
      v_unsigned_32 = v_unsigned_32;
    } else {
      v_unsigned_32 = null;
    }

    let v_unsigned_64 = data["unsigned_64"];

    if (v_unsigned_64 !== null && v_unsigned_64 !== undefined) {
      v_unsigned_64 = v_unsigned_64;
    } else {
      v_unsigned_64 = null;
    }

    let v_signed_32 = data["signed_32"];

    if (v_signed_32 !== null && v_signed_32 !== undefined) {
      v_signed_32 = v_signed_32;
    } else {
      v_signed_32 = null;
    }

    let v_signed_64 = data["signed_64"];

    if (v_signed_64 !== null && v_signed_64 !== undefined) {
      v_signed_64 = v_signed_64;
    } else {
      v_signed_64 = null;
    }

    let v_float_type = data["float_type"];

    if (v_float_type !== null && v_float_type !== undefined) {
      v_float_type = v_float_type;
    } else {
      v_float_type = null;
    }

    let v_double_type = data["double_type"];

    if (v_double_type !== null && v_double_type !== undefined) {
      v_double_type = v_double_type;
    } else {
      v_double_type = null;
    }

    let v_bytes_type = data["bytes_type"];

    if (v_bytes_type !== null && v_bytes_type !== undefined) {
      v_bytes_type = v_bytes_type;
    } else {
      v_bytes_type = null;
    }

    let v_any_type = data["any_type"];

    if (v_any_type !== null && v_any_type !== undefined) {
      v_any_type = v_any_type;
    } else {
      v_any_type = null;
    }

    let v_array_type = data["array_type"];

    if (v_array_type !== null && v_array_type !== undefined) {
      v_array_type = v_array_type.map(function(v) { return Entry.decode(v); });
    } else {
      v_array_type = null;
    }

    let v_map_type = data["map_type"];

    if (v_map_type !== null && v_map_type !== undefined) {
      v_map_type = (function(data) { let o = {}; for (let k in data) { o[k] = Entry.decode(data[k]); }; return o; })(v_map_type);
    } else {
      v_map_type = null;
    }

    return new Entry(v_boolean_type, v_string_type, v_datetime_type, v_unsigned_32, v_unsigned_64, v_signed_32, v_signed_64, v_float_type, v_double_type, v_bytes_type, v_any_type, v_array_type, v_map_type);
  }

  encode() {
    const data = {};

    if (this.boolean_type !== null && this.boolean_type !== undefined) {
      data["boolean_type"] = this.boolean_type;
    }

    if (this.string_type !== null && this.string_type !== undefined) {
      data["string_type"] = this.string_type;
    }

    if (this.datetime_type !== null && this.datetime_type !== undefined) {
      data["datetime_type"] = this.datetime_type;
    }

    if (this.unsigned_32 !== null && this.unsigned_32 !== undefined) {
      data["unsigned_32"] = this.unsigned_32;
    }

    if (this.unsigned_64 !== null && this.unsigned_64 !== undefined) {
      data["unsigned_64"] = this.unsigned_64;
    }

    if (this.signed_32 !== null && this.signed_32 !== undefined) {
      data["signed_32"] = this.signed_32;
    }

    if (this.signed_64 !== null && this.signed_64 !== undefined) {
      data["signed_64"] = this.signed_64;
    }

    if (this.float_type !== null && this.float_type !== undefined) {
      data["float_type"] = this.float_type;
    }

    if (this.double_type !== null && this.double_type !== undefined) {
      data["double_type"] = this.double_type;
    }

    if (this.bytes_type !== null && this.bytes_type !== undefined) {
      data["bytes_type"] = this.bytes_type;
    }

    if (this.any_type !== null && this.any_type !== undefined) {
      data["any_type"] = this.any_type;
    }

    if (this.array_type !== null && this.array_type !== undefined) {
      data["array_type"] = this.array_type.map(function(v) { return v.encode(); });
    }

    if (this.map_type !== null && this.map_type !== undefined) {
      data["map_type"] = (function(data) { let o = {}; for (let k in data) { o[k] = data[k].encode(); }; return o; })(this.map_type);
    }

    return data;
  }
}
