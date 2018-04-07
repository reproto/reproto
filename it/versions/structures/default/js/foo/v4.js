import * as bar from "bar/v1.js";
import * as bar2 from "bar/v2_0.js";
import * as bar21 from "bar/v2_1.js";

export class Thing {
  constructor(name, other, other2, other21) {
    this.name = name;
    this.other = other;
    this.other2 = other2;
    this.other21 = other21;
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

    let v_other21 = data["other21"];

    if (v_other21 !== null && v_other21 !== undefined) {
      v_other21 = bar21.Other.decode(v_other21);
    } else {
      v_other21 = null;
    }

    return new Thing(v_name, v_other, v_other2, v_other21);
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

    if (this.other21 !== null && this.other21 !== undefined) {
      data["other21"] = this.other21.encode();
    }

    return data;
  }
}
