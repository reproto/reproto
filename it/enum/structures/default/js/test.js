export class Entry {
  constructor(explicit, implicit, enum_u32, enum_u64, enum_i32, enum_i64) {
    this.explicit = explicit;
    this.implicit = implicit;
    this.enum_u32 = enum_u32;
    this.enum_u64 = enum_u64;
    this.enum_i32 = enum_i32;
    this.enum_i64 = enum_i64;
  }

  static decode(data) {
    let v_explicit = data["explicit"];

    if (v_explicit !== null && v_explicit !== undefined) {
      v_explicit = EnumExplicit.decode(v_explicit);
    } else {
      v_explicit = null;
    }

    let v_implicit = data["implicit"];

    if (v_implicit !== null && v_implicit !== undefined) {
      v_implicit = EnumImplicit.decode(v_implicit);
    } else {
      v_implicit = null;
    }

    let v_enum_u32 = data["enum_u32"];

    if (v_enum_u32 !== null && v_enum_u32 !== undefined) {
      v_enum_u32 = EnumU32.decode(v_enum_u32);
    } else {
      v_enum_u32 = null;
    }

    let v_enum_u64 = data["enum_u64"];

    if (v_enum_u64 !== null && v_enum_u64 !== undefined) {
      v_enum_u64 = EnumU64.decode(v_enum_u64);
    } else {
      v_enum_u64 = null;
    }

    let v_enum_i32 = data["enum_i32"];

    if (v_enum_i32 !== null && v_enum_i32 !== undefined) {
      v_enum_i32 = EnumI32.decode(v_enum_i32);
    } else {
      v_enum_i32 = null;
    }

    let v_enum_i64 = data["enum_i64"];

    if (v_enum_i64 !== null && v_enum_i64 !== undefined) {
      v_enum_i64 = EnumI64.decode(v_enum_i64);
    } else {
      v_enum_i64 = null;
    }

    return new Entry(v_explicit, v_implicit, v_enum_u32, v_enum_u64, v_enum_i32, v_enum_i64);
  }

  encode() {
    const data = {};

    if (this.explicit !== null && this.explicit !== undefined) {
      data["explicit"] = this.explicit.encode();
    }

    if (this.implicit !== null && this.implicit !== undefined) {
      data["implicit"] = this.implicit.encode();
    }

    if (this.enum_u32 !== null && this.enum_u32 !== undefined) {
      data["enum_u32"] = this.enum_u32.encode();
    }

    if (this.enum_u64 !== null && this.enum_u64 !== undefined) {
      data["enum_u64"] = this.enum_u64.encode();
    }

    if (this.enum_i32 !== null && this.enum_i32 !== undefined) {
      data["enum_i32"] = this.enum_i32.encode();
    }

    if (this.enum_i64 !== null && this.enum_i64 !== undefined) {
      data["enum_i64"] = this.enum_i64.encode();
    }

    return data;
  }
}

export class EnumExplicit {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumExplicit.values.length; i < l; i++) {
      const member = EnumExplicit.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumExplicit.A = new EnumExplicit("A", "foo");
EnumExplicit.B = new EnumExplicit("B", "bar");

EnumExplicit.values = [EnumExplicit.A, EnumExplicit.B];

export class EnumImplicit {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumImplicit.values.length; i < l; i++) {
      const member = EnumImplicit.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumImplicit.A = new EnumImplicit("A", "A");
EnumImplicit.B = new EnumImplicit("B", "B");

EnumImplicit.values = [EnumImplicit.A, EnumImplicit.B];

export class EnumLongNames {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumLongNames.values.length; i < l; i++) {
      const member = EnumLongNames.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumLongNames.FooBar = new EnumLongNames("FooBar", "FooBar");
EnumLongNames.Baz = new EnumLongNames("Baz", "Baz");

EnumLongNames.values = [EnumLongNames.FooBar, EnumLongNames.Baz];

export class EnumU32 {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumU32.values.length; i < l; i++) {
      const member = EnumU32.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumU32.Min = new EnumU32("Min", 0);
EnumU32.Max = new EnumU32("Max", 2147483647);

EnumU32.values = [EnumU32.Min, EnumU32.Max];

export class EnumU64 {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumU64.values.length; i < l; i++) {
      const member = EnumU64.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumU64.Min = new EnumU64("Min", 0);
EnumU64.Max = new EnumU64("Max", 9007199254740991);

EnumU64.values = [EnumU64.Min, EnumU64.Max];

export class EnumI32 {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumI32.values.length; i < l; i++) {
      const member = EnumI32.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumI32.Min = new EnumI32("Min", -2147483648);
EnumI32.NegativeOne = new EnumI32("NegativeOne", -1);
EnumI32.Zero = new EnumI32("Zero", 0);
EnumI32.Max = new EnumI32("Max", 2147483647);

EnumI32.values = [EnumI32.Min, EnumI32.NegativeOne, EnumI32.Zero, EnumI32.Max];

export class EnumI64 {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = EnumI64.values.length; i < l; i++) {
      const member = EnumI64.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

EnumI64.Min = new EnumI64("Min", -9007199254740991);
EnumI64.NegativeOne = new EnumI64("NegativeOne", -1);
EnumI64.Zero = new EnumI64("Zero", 0);
EnumI64.Max = new EnumI64("Max", 9007199254740991);

EnumI64.values = [EnumI64.Min, EnumI64.NegativeOne, EnumI64.Zero, EnumI64.Max];
