
export class Entry {
  constructor() {
  }

  static decode(data) {
    return new Entry();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class Type {
  constructor() {
  }

  static decode(data) {
    return new Type();
  }

  encode() {
    const data = {};

    return data;
  }

  typeMethod() {
  }
}

export class Interface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "SubType") {
      return Interface_SubType.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }

  interfaceMethod() {
  }
}

export class Interface_SubType {
  constructor() {
  }

  static decode(data) {
    return new Interface_SubType();
  }

  encode() {
    const data = {};

    data["type"] = "SubType";

    return data;
  }

  subtypeMethod() {
  }
}

export class Enum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = Enum.values.length; i < l; i++) {
      const member = Enum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }

  enumMethod() {
  }
}

Enum.Variant = new Enum("Variant", "Variant");

Enum.values = [Enum.Variant];

export class Tuple {
  constructor() {
  }

  static decode(data) {
    return new Tuple();
  }

  encode() {
    return [];
  }

  tupleMethod() {
  }
}
