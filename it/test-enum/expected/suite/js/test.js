export class Entry {
  constructor(explicit, implicit) {
    this.explicit = explicit;
    this.implicit = implicit;
  }

  static decode(data) {
    const v_explicit = EnumExplicit.decode(data["explicit"]);

    if (v_explicit === null || v_explicit === undefined) {
      throw new Error("explicit" + ": required field");
    }

    const v_implicit = EnumImplicit.decode(data["implicit"]);

    if (v_implicit === null || v_implicit === undefined) {
      throw new Error("implicit" + ": required field");
    }

    return new Entry(v_explicit, v_implicit);
  }

  encode() {
    const data = {};

    if (this.explicit === null || this.explicit === undefined) {
      throw new Error("explicit: is a required field");
    }

    data["explicit"] = this.explicit.encode();

    if (this.implicit === null || this.implicit === undefined) {
      throw new Error("implicit: is a required field");
    }

    data["implicit"] = this.implicit.encode();

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
