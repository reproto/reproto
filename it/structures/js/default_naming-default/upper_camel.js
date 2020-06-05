export class Value {
  constructor(foo_bar) {
    this.foo_bar = foo_bar;
  }

  static decode(data) {
    const v_foo_bar = data["FooBar"];

    if (v_foo_bar === null || v_foo_bar === undefined) {
      throw new Error("FooBar" + ": required field");
    }

    return new Value(v_foo_bar);
  }

  encode() {
    const data = {};

    if (this.foo_bar === null || this.foo_bar === undefined) {
      throw new Error("FooBar: is a required field");
    }

    data["FooBar"] = this.foo_bar;

    return data;
  }
}
