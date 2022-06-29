export class Value {
  constructor(foo_bar) {
    this.foo_bar = foo_bar;
  }

  static decode(data) {
    let v_foo_bar = data["fooBar"];

    if (v_foo_bar === null || v_foo_bar === undefined) {
      throw new Error("fooBar" + ": required field");
    }

    if (typeof v_foo_bar !== "string") {
      throw Error("expected string");
    }

    return new Value(v_foo_bar);
  }

  encode() {
    const data = {};

    if (this.foo_bar === null || this.foo_bar === undefined) {
      throw new Error("fooBar: is a required field");
    }

    data["fooBar"] = this.foo_bar;

    return data;
  }
}
