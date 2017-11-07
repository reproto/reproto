
export class Entry {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "bar") {
      return Entry_Bar.decode(data);
    }

    if (f_type === "foo") {
      return Entry_Foo.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

export class Entry_Bar {
  constructor() {
  }
  static decode(data) {
    return new Entry_Bar();
  }
  encode() {
    const data = {};

    data["type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_Bar";

export class Entry_Foo {
  constructor() {
  }
  static decode(data) {
    return new Entry_Foo();
  }
  encode() {
    const data = {};

    data["type"] = Entry.TYPE;

    return data;
  }
}

Entry.TYPE = "Entry_Foo";
