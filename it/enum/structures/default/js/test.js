export class Entry {
  constructor(explicit, implicit) {
    this.explicit = explicit;
    this.implicit = implicit;
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

    return new Entry(v_explicit, v_implicit);
  }

  encode() {
    const data = {};

    if (this.explicit !== null && this.explicit !== undefined) {
      data["explicit"] = this.explicit.encode();
    }

    if (this.implicit !== null && this.implicit !== undefined) {
      data["implicit"] = this.implicit.encode();
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
