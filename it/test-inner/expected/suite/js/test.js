export class Entry {
  constructor(a, b) {
    this.a = a;
    this.b = b;
  }

  static decode(data) {
    const v_a = Entry_A.decode(data["a"]);

    if (v_a === null || v_a === undefined) {
      throw new Error("a" + ": required field");
    }

    const v_b = Entry_A_B.decode(data["b"]);

    if (v_b === null || v_b === undefined) {
      throw new Error("b" + ": required field");
    }

    return new Entry(v_a, v_b);
  }

  encode() {
    const data = {};

    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    data["a"] = this.a.encode();

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    data["b"] = this.b.encode();

    return data;
  }
}

export class Entry_A {
  constructor(b) {
    this.b = b;
  }

  static decode(data) {
    const v_b = Entry_A_B.decode(data["b"]);

    if (v_b === null || v_b === undefined) {
      throw new Error("b" + ": required field");
    }

    return new Entry_A(v_b);
  }

  encode() {
    const data = {};

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    data["b"] = this.b.encode();

    return data;
  }
}

export class Entry_A_B {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const v_field = data["field"];

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    return new Entry_A_B(v_field);
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
