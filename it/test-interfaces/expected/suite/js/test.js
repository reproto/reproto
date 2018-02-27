
export class Entry {
  static decode(data) {
    const f_tag = data["@type"]

    if (f_tag === "foo") {
      return Entry_A.decode(data);
    }

    if (f_tag === "b") {
      return Entry_B.decode(data);
    }

    if (f_tag === "Bar") {
      return Entry_Bar.decode(data);
    }

    if (f_tag === "Baz") {
      return Entry_Baz.decode(data);
    }

    throw new Error("bad type: " + f_tag);
  }
}

export class Entry_A {
  constructor(shared) {
    this.shared = shared;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Entry_A(v_shared);
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

Entry.TYPE = "Entry_A";

export class Entry_B {
  constructor(shared) {
    this.shared = shared;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Entry_B(v_shared);
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

Entry.TYPE = "Entry_B";

export class Entry_Bar {
  constructor(shared) {
    this.shared = shared;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Entry_Bar(v_shared);
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

Entry.TYPE = "Entry_Bar";

export class Entry_Baz {
  constructor(shared) {
    this.shared = shared;
  }
  static decode(data) {
    const v_shared = data["shared"];

    if (v_shared === null || v_shared === undefined) {
      throw new Error("shared" + ": required field");
    }

    return new Entry_Baz(v_shared);
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    if (this.shared === null || this.shared === undefined) {
      throw new Error("shared: is a required field");
    }

    data["shared"] = this.shared;

    return data;
  }
}

Entry.TYPE = "Entry_Baz";
