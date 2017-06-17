export class Date {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const v_field = data["field"];

    if (v_field === null || v_field === undefined) {
      throw new Error("field" + ": required field");
    }

    return new Date(v_field);
  }

  encode() {
    const data = {};

    if (this.field === null || this.field === undefined) {
      throw new Error("field: is a required field");
    }

    data["field"] = this.field;

    return data;
  }
}
