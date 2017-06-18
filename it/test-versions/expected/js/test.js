import * as foo from "foo/4_0_0.js";

export class Entry {
  constructor(thing) {
    this.thing = thing;
  }

  static decode(data) {
    let v_thing = data["thing"];

    if (v_thing !== null && v_thing !== undefined) {
      v_thing = foo.Thing.decode(v_thing);
    } else {
      v_thing = null;
    }

    return new Entry(v_thing);
  }

  encode() {
    const data = {};

    if (this.thing !== null && this.thing !== undefined) {
      data["thing"] = this.thing.encode();
    }

    return data;
  }
}
