
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

export class RootType {
  constructor() {
  }

  static decode(data) {
    return new RootType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "Foo") {
      return RootInterface_Foo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootInterface_Foo {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo();
  }

  encode() {
    const data = {};

    data["type"] = "Foo";

    return data;
  }
}

export class RootEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootEnum.values.length; i < l; i++) {
      const member = RootEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootEnum.Foo = new RootEnum("Foo", "Foo");

RootEnum.values = [RootEnum.Foo];

export class RootTuple {
  constructor() {
  }

  static decode(data) {
    return new RootTuple();
  }

  encode() {
    return [];
  }
}

export class RootType_NestedType {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootType_NestedInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "Foo") {
      return RootType_NestedInterface_Foo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootType_NestedInterface_Foo {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedInterface_Foo();
  }

  encode() {
    const data = {};

    data["type"] = "Foo";

    return data;
  }
}

export class RootType_NestedEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootType_NestedEnum.values.length; i < l; i++) {
      const member = RootType_NestedEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootType_NestedEnum.Foo = new RootType_NestedEnum("Foo", "Foo");

RootType_NestedEnum.values = [RootType_NestedEnum.Foo];

export class RootType_NestedTuple {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedTuple();
  }

  encode() {
    return [];
  }
}

export class RootInterface_Foo_NestedType {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootInterface_Foo_NestedInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "NestedFoo") {
      return RootInterface_Foo_NestedInterface_NestedFoo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootInterface_Foo_NestedInterface_NestedFoo {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedInterface_NestedFoo();
  }

  encode() {
    const data = {};

    data["type"] = "NestedFoo";

    return data;
  }
}

export class RootInterface_Foo_NestedEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootInterface_Foo_NestedEnum.values.length; i < l; i++) {
      const member = RootInterface_Foo_NestedEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootInterface_Foo_NestedEnum.Foo = new RootInterface_Foo_NestedEnum("Foo", "Foo");

RootInterface_Foo_NestedEnum.values = [RootInterface_Foo_NestedEnum.Foo];

export class RootInterface_Foo_NestedTuple {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedTuple();
  }

  encode() {
    return [];
  }
}

export class RootTuple_NestedType {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootTuple_NestedInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "Foo") {
      return RootTuple_NestedInterface_Foo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootTuple_NestedInterface_Foo {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedInterface_Foo();
  }

  encode() {
    const data = {};

    data["type"] = "Foo";

    return data;
  }
}

export class RootTuple_NestedEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootTuple_NestedEnum.values.length; i < l; i++) {
      const member = RootTuple_NestedEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootTuple_NestedEnum.Foo = new RootTuple_NestedEnum("Foo", "Foo");

RootTuple_NestedEnum.values = [RootTuple_NestedEnum.Foo];

export class RootTuple_NestedTuple {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedTuple();
  }

  encode() {
    return [];
  }
}

export class RootService_NestedType {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootService_NestedInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "Foo") {
      return RootService_NestedInterface_Foo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootService_NestedInterface_Foo {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedInterface_Foo();
  }

  encode() {
    const data = {};

    data["type"] = "Foo";

    return data;
  }
}

export class RootService_NestedEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootService_NestedEnum.values.length; i < l; i++) {
      const member = RootService_NestedEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootService_NestedEnum.Foo = new RootService_NestedEnum("Foo", "Foo");

RootService_NestedEnum.values = [RootService_NestedEnum.Foo];

export class RootService_NestedTuple {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedTuple();
  }

  encode() {
    return [];
  }
}

export class RootType_NestedInterface_Foo_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedInterface_Foo_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootType_NestedTuple_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedTuple_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootType_NestedService_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedService_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootInterface_Foo_NestedInterface_NestedFoo_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedInterface_NestedFoo_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootInterface_Foo_NestedTuple_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedTuple_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootInterface_Foo_NestedService_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedService_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootTuple_NestedInterface_Foo_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedInterface_Foo_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootTuple_NestedTuple_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedTuple_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootTuple_NestedService_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedService_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootService_NestedInterface_Foo_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedInterface_Foo_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootService_NestedTuple_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedTuple_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootService_NestedService_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedService_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}
