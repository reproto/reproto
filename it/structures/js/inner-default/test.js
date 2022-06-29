export class Entry {
  constructor(a, b) {
    this.a = a;
    this.b = b;
  }

  static decode(data) {
    let v_a = data["a"];

    if (v_a !== null && v_a !== undefined) {
      v_a = A.decode(v_a);
    } else {
      v_a = null;
    }

    let v_b = data["b"];

    if (v_b !== null && v_b !== undefined) {
      v_b = A_B.decode(v_b);
    } else {
      v_b = null;
    }

    return new Entry(v_a, v_b);
  }

  encode() {
    const data = {};

    if (this.a !== null && this.a !== undefined) {
      data["a"] = this.a.encode();
    }

    if (this.b !== null && this.b !== undefined) {
      data["b"] = this.b.encode();
    }

    return data;
  }
}

export class A {
  constructor(b) {
    this.b = b;
  }

  static decode(data) {
    let v_b = data["b"];

    if (v_b === null || v_b === undefined) {
      throw new Error("b" + ": required field");
    }

    v_b = A_B.decode(v_b);

    return new A(v_b);
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

export class A_B {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    let v_field = data["field"];

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    if (typeof v_field !== "string") {
      throw Error("expected string");
    }

    return new A_B(v_field);
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
