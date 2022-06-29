export class Entry {
  constructor(boolean_type, string_type, datetime_type, unsigned_32, unsigned_64, signed_32, signed_64, float_type, double_type, bytes_type, any_type, array_type, array_of_array_type, map_type) {
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
    this.array_of_array_type = array_of_array_type;
    this.map_type = map_type;
  }

  static decode(data) {
    let v_boolean_type = data["boolean_type"];

    if (v_boolean_type !== null && v_boolean_type !== undefined) {
      if (typeof v_boolean_type !== "boolean") {
        throw Error("expected boolean");
      }
    } else {
      v_boolean_type = null;
    }

    let v_string_type = data["string_type"];

    if (v_string_type !== null && v_string_type !== undefined) {
      if (typeof v_string_type !== "string") {
        throw Error("expected string");
      }
    } else {
      v_string_type = null;
    }

    let v_datetime_type = data["datetime_type"];

    if (v_datetime_type !== null && v_datetime_type !== undefined) {
      if (typeof v_datetime_type !== "string") {
        throw Error("expected string");
      }
    } else {
      v_datetime_type = null;
    }

    let v_unsigned_32 = data["unsigned_32"];

    if (v_unsigned_32 !== null && v_unsigned_32 !== undefined) {
      if (!Number.isInteger(v_unsigned_32)) {
        throw Error("expected integer");
      }
    } else {
      v_unsigned_32 = null;
    }

    let v_unsigned_64 = data["unsigned_64"];

    if (v_unsigned_64 !== null && v_unsigned_64 !== undefined) {
      if (!Number.isInteger(v_unsigned_64)) {
        throw Error("expected integer");
      }
    } else {
      v_unsigned_64 = null;
    }

    let v_signed_32 = data["signed_32"];

    if (v_signed_32 !== null && v_signed_32 !== undefined) {
      if (!Number.isInteger(v_signed_32)) {
        throw Error("expected integer");
      }
    } else {
      v_signed_32 = null;
    }

    let v_signed_64 = data["signed_64"];

    if (v_signed_64 !== null && v_signed_64 !== undefined) {
      if (!Number.isInteger(v_signed_64)) {
        throw Error("expected integer");
      }
    } else {
      v_signed_64 = null;
    }

    let v_float_type = data["float_type"];

    if (v_float_type !== null && v_float_type !== undefined) {
      if (!Number.isFinite(v_float_type)) {
        throw Error("expected float");
      }
    } else {
      v_float_type = null;
    }

    let v_double_type = data["double_type"];

    if (v_double_type !== null && v_double_type !== undefined) {
      if (!Number.isFinite(v_double_type)) {
        throw Error("expected float");
      }
    } else {
      v_double_type = null;
    }

    let v_bytes_type = data["bytes_type"];

    if (v_bytes_type !== null && v_bytes_type !== undefined) {
      if (typeof v_bytes_type !== "string") {
        throw Error("expected string");
      }
    } else {
      v_bytes_type = null;
    }

    let v_any_type = data["any_type"];

    if (v_any_type !== null && v_any_type !== undefined) {} else {
      v_any_type = null;
    }

    let v_array_type = data["array_type"];

    if (v_array_type !== null && v_array_type !== undefined) {
      if (!Array.isArray(v_array_type)) {
        throw Error("expected array");
      }

      let o0 = [];

      for (let i0 = 0, l0 = v_array_type.length; i0 < l0; i0++) {
        let v0 = v_array_type[i0];

        v0 = Entry.decode(v0);

        o0.push(v0);
      }

      v_array_type = o0;
    } else {
      v_array_type = null;
    }

    let v_array_of_array_type = data["array_of_array_type"];

    if (v_array_of_array_type !== null && v_array_of_array_type !== undefined) {
      if (!Array.isArray(v_array_of_array_type)) {
        throw Error("expected array");
      }

      let o0 = [];

      for (let i0 = 0, l0 = v_array_of_array_type.length; i0 < l0; i0++) {
        let v0 = v_array_of_array_type[i0];

        if (!Array.isArray(v0)) {
          throw Error("expected array");
        }

        let o1 = [];

        for (let i1 = 0, l1 = v0.length; i1 < l1; i1++) {
          let v1 = v0[i1];

          v1 = Entry.decode(v1);

          o1.push(v1);
        }

        v0 = o1;

        o0.push(v0);
      }

      v_array_of_array_type = o0;
    } else {
      v_array_of_array_type = null;
    }

    let v_map_type = data["map_type"];

    if (v_map_type !== null && v_map_type !== undefined) {
      if (typeof v_map_type !== "object") {
        throw Error("expected object");
      }

      let o0 = {};

      for (let [k0, v0] of Object.entries(v_map_type)) {
        if (typeof k0 !== "string") {
          throw Error("expected string");
        }
        v0 = Entry.decode(v0);

        o0[k0] = v0;
      }

      v_map_type = o0;
    } else {
      v_map_type = null;
    }

    return new Entry(v_boolean_type, v_string_type, v_datetime_type, v_unsigned_32, v_unsigned_64, v_signed_32, v_signed_64, v_float_type, v_double_type, v_bytes_type, v_any_type, v_array_type, v_array_of_array_type, v_map_type);
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

    if (this.array_of_array_type !== null && this.array_of_array_type !== undefined) {
      data["array_of_array_type"] = this.array_of_array_type.map(function(v) { return v.map(function(v) { return v.encode(); }); });
    }

    if (this.map_type !== null && this.map_type !== undefined) {
      data["map_type"] = (function(data) {
        let o = {};

        for (let k in data) {
          o[k] = data[k].encode();
        }

        return o;
      })(this.map_type);
    }

    return data;
  }
}
