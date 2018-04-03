
export class Other {
  constructor(name21) {
    this.name21 = name21;
  }

  static decode(data) {
    const v_name21 = data["name21"];

    if (v_name21 === null || v_name21 === undefined) {
      throw new Error("name21" + ": required field");
    }

    return new Other(v_name21);
  }

  encode() {
    const data = {};

    if (this.name21 === null || this.name21 === undefined) {
      throw new Error("name21: is a required field");
    }

    data["name21"] = this.name21;

    return data;
  }
}
