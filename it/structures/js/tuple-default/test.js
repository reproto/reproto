export class Entry {
  constructor(tuple1, tuple2) {
    this.tuple1 = tuple1;
    this.tuple2 = tuple2;
  }

  static decode(data) {
    let v_tuple1 = data["tuple1"];

    if (v_tuple1 !== null && v_tuple1 !== undefined) {
      v_tuple1 = Tuple1.decode(v_tuple1);
    } else {
      v_tuple1 = null;
    }

    let v_tuple2 = data["tuple2"];

    if (v_tuple2 !== null && v_tuple2 !== undefined) {
      v_tuple2 = Tuple2.decode(v_tuple2);
    } else {
      v_tuple2 = null;
    }

    return new Entry(v_tuple1, v_tuple2);
  }

  encode() {
    const data = {};

    if (this.tuple1 !== null && this.tuple1 !== undefined) {
      data["tuple1"] = this.tuple1.encode();
    }

    if (this.tuple2 !== null && this.tuple2 !== undefined) {
      data["tuple2"] = this.tuple2.encode();
    }

    return data;
  }
}

export class Tuple1 {
  constructor(a, b) {
    this.a = a;
    this.b = b;
  }

  static decode(data) {
    let v_a = data[0];

    if (v_a === null || v_a === undefined) {
      throw new Error(0 + ": required field");
    }

    if (typeof v_a !== "string") {
      throw Error("expected string");
    }

    let v_b = data[1];

    if (v_b === null || v_b === undefined) {
      throw new Error(1 + ": required field");
    }

    if (!Number.isInteger(v_b)) {
      throw Error("expected integer");
    }

    return new Tuple1(v_a, v_b);
  }

  encode() {
    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    return [this.a, this.b];
  }
}

export class Tuple2 {
  constructor(a, b) {
    this.a = a;
    this.b = b;
  }

  static decode(data) {
    let v_a = data[0];

    if (v_a === null || v_a === undefined) {
      throw new Error(0 + ": required field");
    }

    if (typeof v_a !== "string") {
      throw Error("expected string");
    }

    let v_b = data[1];

    if (v_b === null || v_b === undefined) {
      throw new Error(1 + ": required field");
    }

    v_b = Other.decode(v_b);

    return new Tuple2(v_a, v_b);
  }

  encode() {
    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    return [this.a, this.b.encode()];
  }
}

export class Other {
  constructor(a) {
    this.a = a;
  }

  static decode(data) {
    let v_a = data["a"];

    if (v_a === null || v_a === undefined) {
      throw new Error("a" + ": required field");
    }

    if (typeof v_a !== "string") {
      throw Error("expected string");
    }

    return new Other(v_a);
  }

  encode() {
    const data = {};

    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    data["a"] = this.a;

    return data;
  }
}
