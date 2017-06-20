import * as bar from "bar/_1_0_0.js";
import * as bar2 from "bar/_2_0_0.js";

export class Thing {
  constructor(name, other, other2) {
    this.name = name;
    this.other = other;
    this.other2 = other2;
  }

  static decode(data) {
    const v_name = data["name"];

    if (v_name === null || v_name === undefined) {
      throw new Error("name" + ": required field");
    }

    const v_other = bar.Other.decode(data["other"]);

    if (v_other === null || v_other === undefined) {
      throw new Error("other" + ": required field");
    }

    const v_other2 = bar2.Other.decode(data["other2"]);

    if (v_other2 === null || v_other2 === undefined) {
      throw new Error("other2" + ": required field");
    }

    return new Thing(v_name, v_other, v_other2);
  }

  encode() {
    const data = {};

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    if (this.other === null || this.other === undefined) {
      throw new Error("other: is a required field");
    }

    data["other"] = this.other.encode();

    if (this.other2 === null || this.other2 === undefined) {
      throw new Error("other2: is a required field");
    }

    data["other2"] = this.other2.encode();

    return data;
  }
}
