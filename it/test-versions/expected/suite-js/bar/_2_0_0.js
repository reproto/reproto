export class Other {
  constructor(name2) {
    this.name2 = name2;
  }

  static decode(data) {
    const v_name2 = data["name2"];

    if (v_name2 === null || v_name2 === undefined) {
      throw new Error("name2" + ": required field");
    }

    return new Other(v_name2);
  }

  encode() {
    const data = {};

    if (this.name2 === null || this.name2 === undefined) {
      throw new Error("name2: is a required field");
    }

    data["name2"] = this.name2;

    return data;
  }
}
