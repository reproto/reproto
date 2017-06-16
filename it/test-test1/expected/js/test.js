class Foo {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const field = data["field"];

    return new Foo(field);
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
