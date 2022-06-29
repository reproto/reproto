export class Value {
  constructor(foo_bar) {
    this.foo_bar = foo_bar;
  }

  static decode(data) {
    let v_foo_bar = data["FOO_BAR"];

    if (v_foo_bar === null || v_foo_bar === undefined) {
      throw new Error("FOO_BAR" + ": required field");
    }

    if (typeof v_foo_bar !== "string") {
      throw Error("expected string");
    }

    return new Value(v_foo_bar);
  }

  encode() {
    const data = {};

    if (this.foo_bar === null || this.foo_bar === undefined) {
      throw new Error("FOO_BAR: is a required field");
    }

    data["FOO_BAR"] = this.foo_bar;

    return data;
  }
}
