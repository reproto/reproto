export class Entry {
  constructor(tagged, required_fields) {
    this.tagged = tagged;
    this.required_fields = required_fields;
  }

  static decode(data) {
    let v_tagged = data["tagged"];

    if (v_tagged !== null && v_tagged !== undefined) {
      v_tagged = Tagged.decode(v_tagged);
    } else {
      v_tagged = null;
    }

    let v_required_fields = data["required_fields"];

    if (v_required_fields !== null && v_required_fields !== undefined) {
      v_required_fields = RequiredFields.decode(v_required_fields);
    } else {
      v_required_fields = null;
    }

    return new Entry(v_tagged, v_required_fields);
  }

  encode() {
    const data = {};

    if (this.tagged !== null && this.tagged !== undefined) {
      data["tagged"] = this.tagged.encode();
    }

    if (this.required_fields !== null && this.required_fields !== undefined) {
      data["required_fields"] = this.required_fields.encode();
    }

    return data;
  }
}

export class Tagged {
  static decode(data) {
    const f_tag = data["@type"]

    if (f_tag === "foo") {
      return Tagged_A.decode(data);
    }

    if (f_tag === "b") {
      return Tagged_B.decode(data);
    }

    if (f_tag === "Bar") {
      return Tagged_Bar.decode(data);
    }

    if (f_tag === "Baz") {
      return Tagged_Baz.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class Tagged_A {
  constructor(shared) {
    this.shared = shared;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Tagged_A(v_shared);
  }

  encode() {
    const data = {};

    data["@type"] = "foo";

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

export class Tagged_B {
  constructor(shared) {
    this.shared = shared;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Tagged_B(v_shared);
  }

  encode() {
    const data = {};

    data["@type"] = "b";

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

export class Tagged_Bar {
  constructor(shared) {
    this.shared = shared;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Tagged_Bar(v_shared);
  }

  encode() {
    const data = {};

    data["@type"] = "Bar";

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

export class Tagged_Baz {
  constructor(shared) {
    this.shared = shared;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Tagged_Baz(v_shared);
  }

  encode() {
    const data = {};

    data["@type"] = "Baz";

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

export class RequiredFields {
  static decode(data) {
    var all = true

    var keys = {}

    for (const k in data) {
      keys[k] = true
    }

    if (("shared" in keys) && ("a" in keys) && ("b" in keys)) {
      return RequiredFields_A.decode(data);
    }

    if (("shared" in keys) && ("a" in keys)) {
      return RequiredFields_B.decode(data);
    }

    if (("shared" in keys) && ("b" in keys)) {
      return RequiredFields_C.decode(data);
    }

    throw new Error("no legal field combinations found");
  }
}

export class RequiredFields_A {
  constructor(shared, shared_ignore, a, b, ignore) {
    this.shared = shared;
    this.shared_ignore = shared_ignore;
    this.a = a;
    this.b = b;
    this.ignore = ignore;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    let v_shared_ignore = data["shared_ignore"];

    if (v_shared_ignore !== null && v_shared_ignore !== undefined) {
      v_shared_ignore = v_shared_ignore;
    } else {
      v_shared_ignore = null;
    }

    const v_a = data["a"];

    if (v_a === null || v_a === undefined) {
      throw new Error("a" + ": required field");
    }

    const v_b = data["b"];

    if (v_b === null || v_b === undefined) {
      throw new Error("b" + ": required field");
    }

    let v_ignore = data["ignore"];

    if (v_ignore !== null && v_ignore !== undefined) {
      v_ignore = v_ignore;
    } else {
      v_ignore = null;
    }

    return new RequiredFields_A(v_shared, v_shared_ignore, v_a, v_b, v_ignore);
  }

  encode() {
    const data = {};

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    if (this.shared_ignore !== null && this.shared_ignore !== undefined) {
      data["shared_ignore"] = this.shared_ignore;
    }

    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    data["a"] = this.a;

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    data["b"] = this.b;

    if (this.ignore !== null && this.ignore !== undefined) {
      data["ignore"] = this.ignore;
    }

    return data;
  }
}

export class RequiredFields_B {
  constructor(shared, shared_ignore, a, ignore) {
    this.shared = shared;
    this.shared_ignore = shared_ignore;
    this.a = a;
    this.ignore = ignore;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    let v_shared_ignore = data["shared_ignore"];

    if (v_shared_ignore !== null && v_shared_ignore !== undefined) {
      v_shared_ignore = v_shared_ignore;
    } else {
      v_shared_ignore = null;
    }

    const v_a = data["a"];

    if (v_a === null || v_a === undefined) {
      throw new Error("a" + ": required field");
    }

    let v_ignore = data["ignore"];

    if (v_ignore !== null && v_ignore !== undefined) {
      v_ignore = v_ignore;
    } else {
      v_ignore = null;
    }

    return new RequiredFields_B(v_shared, v_shared_ignore, v_a, v_ignore);
  }

  encode() {
    const data = {};

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    if (this.shared_ignore !== null && this.shared_ignore !== undefined) {
      data["shared_ignore"] = this.shared_ignore;
    }

    if (this.a === null || this.a === undefined) {
      throw new Error("a: is a required field");
    }

    data["a"] = this.a;

    if (this.ignore !== null && this.ignore !== undefined) {
      data["ignore"] = this.ignore;
    }

    return data;
  }
}

export class RequiredFields_C {
  constructor(shared, shared_ignore, b, ignore) {
    this.shared = shared;
    this.shared_ignore = shared_ignore;
    this.b = b;
    this.ignore = ignore;
  }

  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    let v_shared_ignore = data["shared_ignore"];

    if (v_shared_ignore !== null && v_shared_ignore !== undefined) {
      v_shared_ignore = v_shared_ignore;
    } else {
      v_shared_ignore = null;
    }

    const v_b = data["b"];

    if (v_b === null || v_b === undefined) {
      throw new Error("b" + ": required field");
    }

    let v_ignore = data["ignore"];

    if (v_ignore !== null && v_ignore !== undefined) {
      v_ignore = v_ignore;
    } else {
      v_ignore = null;
    }

    return new RequiredFields_C(v_shared, v_shared_ignore, v_b, v_ignore);
  }

  encode() {
    const data = {};

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    if (this.shared_ignore !== null && this.shared_ignore !== undefined) {
      data["shared_ignore"] = this.shared_ignore;
    }

    if (this.b === null || this.b === undefined) {
      throw new Error("b: is a required field");
    }

    data["b"] = this.b;

    if (this.ignore !== null && this.ignore !== undefined) {
      data["ignore"] = this.ignore;
    }

    return data;
  }
}
