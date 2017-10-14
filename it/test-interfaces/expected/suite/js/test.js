
export class Entry {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "bar") {
      return Entry_Bar.decode(data);
    }

    if (f_type === "foo") {
      return Entry_Foo.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

export class Entry_Bar {
  constructor(shared, bar) {
    this.shared = shared;
    this.bar = bar;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    const v_bar = data["bar"];

    if (v_bar === null || v_bar === undefined) {
      throw new Error("bar" + ": required field");
    }

    return new Entry_Bar(v_shared, v_bar);
  }
  encode() {
    const data = {};

    data["type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    if (this.bar === null || this.bar === undefined) {
      throw new Error("bar: is a required field");
    }

    data["bar"] = this.bar;

    return data;
  }
}

Entry.TYPE = "Entry_Bar";

export class Entry_Foo {
  constructor(shared, foo) {
    this.shared = shared;
    this.foo = foo;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    const v_foo = data["foo"];

    if (v_foo === null || v_foo === undefined) {
      throw new Error("foo" + ": required field");
    }

    return new Entry_Foo(v_shared, v_foo);
  }
  encode() {
    const data = {};

    data["type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    if (this.foo === null || this.foo === undefined) {
      throw new Error("foo: is a required field");
    }

    data["foo"] = this.foo;

    return data;
  }
}

Entry.TYPE = "Entry_Foo";
