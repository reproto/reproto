
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

    data["type"] = RootInterface.TYPE;

    return data;
  }
}

RootInterface.TYPE = "RootInterface_Foo";

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

    data["type"] = RootType_NestedInterface.TYPE;

    return data;
  }
}

RootType_NestedInterface.TYPE = "RootType_NestedInterface_Foo";

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

    data["type"] = RootInterface_Foo_NestedInterface.TYPE;

    return data;
  }
}

RootInterface_Foo_NestedInterface.TYPE = "RootInterface_Foo_NestedInterface_NestedFoo";

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

export class RootEnum_NestedType {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedType();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootEnum_NestedInterface {
  static decode(data) {
    const f_tag = data["type"]

    if (f_tag === "Foo") {
      return RootEnum_NestedInterface_Foo.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class RootEnum_NestedInterface_Foo {
  constructor() {
  }
  static decode(data) {
    return new RootEnum_NestedInterface_Foo();
  }
  encode() {
    const data = {};

    data["type"] = RootEnum_NestedInterface.TYPE;

    return data;
  }
}

RootEnum_NestedInterface.TYPE = "RootEnum_NestedInterface_Foo";

export class RootEnum_NestedEnum {
  constructor(name, value) {
    this.name = name;
    this.value = value;
  }

  encode() {
    return this.value;
  }
  static decode(data) {
    for (let i = 0, l = RootEnum_NestedEnum.values.length; i < l; i++) {
      const member = RootEnum_NestedEnum.values[i]



      if (member.value === data) {
        return member;
      }
    }

    throw new Error("no matching value: " + data);
  }
}

RootEnum_NestedEnum.Foo = new RootEnum_NestedEnum("Foo", "Foo");

RootEnum_NestedEnum.values = [RootEnum_NestedEnum.Foo];

export class RootEnum_NestedTuple {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedTuple();
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

    data["type"] = RootTuple_NestedInterface.TYPE;

    return data;
  }
}

RootTuple_NestedInterface.TYPE = "RootTuple_NestedInterface_Foo";

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

    data["type"] = RootService_NestedInterface.TYPE;

    return data;
  }
}

RootService_NestedInterface.TYPE = "RootService_NestedInterface_Foo";

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

export class RootType_NestedEnum_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootType_NestedEnum_Nested();
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

export class RootInterface_Foo_NestedEnum_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootInterface_Foo_NestedEnum_Nested();
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

export class RootEnum_NestedInterface_Foo_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedInterface_Foo_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootEnum_NestedEnum_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedEnum_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootEnum_NestedTuple_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedTuple_Nested();
  }

  encode() {
    const data = {};

    return data;
  }
}

export class RootEnum_NestedService_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootEnum_NestedService_Nested();
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

export class RootTuple_NestedEnum_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootTuple_NestedEnum_Nested();
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

export class RootService_NestedEnum_Nested {
  constructor() {
  }

  static decode(data) {
    return new RootService_NestedEnum_Nested();
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
