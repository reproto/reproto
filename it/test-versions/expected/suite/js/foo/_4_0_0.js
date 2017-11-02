import * as bar from "bar/_1_0_0.js";
import * as bar2 from "bar/_2_0_0.js";

export class Thing {
  constructor(name, other, other2) {
    this.name = name;
    this.other = other;
    this.other2 = other2;
  }

  static decode(data) {
    let v_name = data["name"];

    if (v_name !== null && v_name !== undefined) {
      v_name = v_name;
    } else {
      v_name = null;
    }

    let v_other = data["other"];

    if (v_other !== null && v_other !== undefined) {
      v_other = bar.Other.decode(v_other);
    } else {
      v_other = null;
    }

    let v_other2 = data["other2"];

    if (v_other2 !== null && v_other2 !== undefined) {
      v_other2 = bar2.Other.decode(v_other2);
    } else {
      v_other2 = null;
    }

    return new Thing(v_name, v_other, v_other2);
  }

  encode() {
    const data = {};

    if (this.name !== null && this.name !== undefined) {
      data["name"] = this.name;
    }

    if (this.other !== null && this.other !== undefined) {
      data["other"] = this.other.encode();
    }

    if (this.other2 !== null && this.other2 !== undefined) {
      data["other2"] = this.other2.encode();
    }

    return data;
  }
}
