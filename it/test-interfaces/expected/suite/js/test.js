
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
  constructor() {
  }
  static decode(data) {
    return new Entry_A();
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_A";

export class Entry_B {
  constructor() {
  }
  static decode(data) {
    return new Entry_B();
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_B";

export class Entry_Bar {
  constructor() {
  }
  static decode(data) {
    return new Entry_Bar();
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_Bar";

export class Entry_Baz {
  constructor() {
  }
  static decode(data) {
    return new Entry_Baz();
  }
  encode() {
    const data = {};

    data["@type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_Baz";
