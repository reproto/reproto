export class Thing {
  constructor(name) {
    this.name = name;
  }

  static decode(data) {
    const v_name = data["name"];

    if (v_name === null || v_name === undefined) {
      throw new Error("name" + ": required field");
    }

    return new Thing(v_name);
  }

  encode() {
    const data = {};

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    return data;
  }
}
