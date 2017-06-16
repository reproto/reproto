class Date {
  constructor(field) {
    this.field = field;
  }

  static decode(data) {
    const field = data["field"];

    return new Date(field);
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
