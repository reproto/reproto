export class Entry {
  constructor(boolean_type, string_type, unsigned_type, unsigned_sized_type, signed_type, signed_sized_type, float_type, double_type, bytes_type, any_type, array_type, map_type) {
    this.boolean_type = boolean_type;
    this.string_type = string_type;
    this.unsigned_type = unsigned_type;
    this.unsigned_sized_type = unsigned_sized_type;
    this.signed_type = signed_type;
    this.signed_sized_type = signed_sized_type;
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

    let v_unsigned_type = data["unsigned_type"];

    if (v_unsigned_type !== null && v_unsigned_type !== undefined) {
      v_unsigned_type = v_unsigned_type;
    } else {
      v_unsigned_type = null;
    }

    let v_unsigned_sized_type = data["unsigned_sized_type"];

    if (v_unsigned_sized_type !== null && v_unsigned_sized_type !== undefined) {
      v_unsigned_sized_type = v_unsigned_sized_type;
    } else {
      v_unsigned_sized_type = null;
    }

    let v_signed_type = data["signed_type"];

    if (v_signed_type !== null && v_signed_type !== undefined) {
      v_signed_type = v_signed_type;
    } else {
      v_signed_type = null;
    }

    let v_signed_sized_type = data["signed_sized_type"];

    if (v_signed_sized_type !== null && v_signed_sized_type !== undefined) {
      v_signed_sized_type = v_signed_sized_type;
    } else {
      v_signed_sized_type = null;
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

    return new Entry(v_boolean_type, v_string_type, v_unsigned_type, v_unsigned_sized_type, v_signed_type, v_signed_sized_type, v_float_type, v_double_type, v_bytes_type, v_any_type, v_array_type, v_map_type);
  }

  encode() {
    const data = {};

    if (this.boolean_type !== null && this.boolean_type !== undefined) {
      data["boolean_type"] = this.boolean_type;
    }

    if (this.string_type !== null && this.string_type !== undefined) {
      data["string_type"] = this.string_type;
    }

    if (this.unsigned_type !== null && this.unsigned_type !== undefined) {
      data["unsigned_type"] = this.unsigned_type;
    }

    if (this.unsigned_sized_type !== null && this.unsigned_sized_type !== undefined) {
      data["unsigned_sized_type"] = this.unsigned_sized_type;
    }

    if (this.signed_type !== null && this.signed_type !== undefined) {
      data["signed_type"] = this.signed_type;
    }

    if (this.signed_sized_type !== null && this.signed_sized_type !== undefined) {
      data["signed_sized_type"] = this.signed_sized_type;
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
