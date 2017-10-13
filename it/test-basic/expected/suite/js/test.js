
export class Entry {
  constructor(foo) {
    this.foo = foo;
  }

  static decode(data) {
    let v_foo = data["foo"];

    if (v_foo !== null && v_foo !== undefined) {
      v_foo = Foo.decode(v_foo);
    } else {
      v_foo = null;
    }

    return new Entry(v_foo);
  }

  encode() {
    const data = {};

    if (this.foo !== null && this.foo !== undefined) {
      data["foo"] = this.foo.encode();
    }

    return data;
  }
}

export class Foo {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const v_field = data["field"];

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    return new Foo(v_field);
  }

  encode() {
    const data = {};

    if (this.field === null || this.field === undefined) {
      throw new Error("field: is a required field");
    }

    data["field"] = this.field;

    return data;
  }
}

export class Bar {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const v_field = Bar_Inner.decode(data["field"]);

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    return new Bar(v_field);
  }

  encode() {
    const data = {};

    if (this.field === null || this.field === undefined) {
      throw new Error("field: is a required field");
    }

    data["field"] = this.field.encode();

    return data;
  }
}

export class Bar_Inner {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const v_field = data["field"];

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    return new Bar_Inner(v_field);
  }

  encode() {
    const data = {};

    if (this.field === null || this.field === undefined) {
      throw new Error("field: is a required field");
    }

    data["field"] = this.field;

    return data;
  }
}
