export class Entry {
  constructor(data, point, interface_field, type_field) {
    this.data = data;
    this.point = point;
    this.interface_field = interface_field;
    this.type_field = type_field;
  }

  static decode(data) {
    let v_data = data["data"];

    if (v_data !== null && v_data !== undefined) {
      v_data = Data.decode(v_data);
    } else {
      v_data = null;
    }

    let v_point = data["point"];

    if (v_point !== null && v_point !== undefined) {
      v_point = Point.decode(v_point);
    } else {
      v_point = null;
    }

    let v_interface_field = data["interface"];

    if (v_interface_field !== null && v_interface_field !== undefined) {
      v_interface_field = Interface.decode(v_interface_field);
    } else {
      v_interface_field = null;
    }

    let v_type_field = data["type"];

    if (v_type_field !== null && v_type_field !== undefined) {
      v_type_field = Type.decode(v_type_field);
    } else {
      v_type_field = null;
    }

    return new Entry(v_data, v_point, v_interface_field, v_type_field);
  }

  encode() {
    const data = {};

    if (this.data !== null && this.data !== undefined) {
      data["data"] = this.data.encode();
    }

    if (this.point !== null && this.point !== undefined) {
      data["point"] = this.point.encode();
    }

    if (this.interface_field !== null && this.interface_field !== undefined) {
      data["interface"] = this.interface_field.encode();
    }

    if (this.type_field !== null && this.type_field !== undefined) {
      data["type"] = this.type_field.encode();
    }

    return data;
  }
}

export class Data {
  constructor(name) {
    this.name = name;
  }

  static decode(data) {
    const v_name = data["name"];

    if (v_name === null || v_name === undefined) {
      throw new Error("name" + ": required field");
    }

    return new Data(v_name);
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

export class Point {
  constructor(timestamp, value) {
    this.timestamp = timestamp;
    this.value = value;
  }

  static decode(data) {
    if (data == 42) {
      return new Point(42, 41.2)
    }

    if (typeof data === "number") {
      n = data
      return new Point(n, 42);
    }

    const v_timestamp = data[0];

    if (v_timestamp === null || v_timestamp === undefined) {
      throw new Error(0 + ": required field");
    }

    const v_value = data[1];

    if (v_value === null || v_value === undefined) {
      throw new Error(1 + ": required field");
    }

    return new Point(v_timestamp, v_value);
  }

  encode() {
    if (this.timestamp === null || this.timestamp === undefined) {
      throw new Error("TS: is a required field");
    }

    if (this.value === null || this.value === undefined) {
      throw new Error("value: is a required field");
    }

    return [this.timestamp, this.value];
  }
}

export class Interface {
  static decode(data) {
    if (typeof data === "string") {
      name = data
      return new Interface_One(name, null, new Data("data"));
    }

    const f_type = data["type"]

    if (f_type === "one") {
      return Interface_One.decode(data);
    }

    if (f_type === "two") {
      return Interface_Two.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

export class Interface_One {
  constructor(name, other, data) {
    this.name = name;
    this.other = other;
    this.data = data;
  }

  static decode(data) {
    const v_name = data["name"];

    if (v_name === null || v_name === undefined) {
      throw new Error("name" + ": required field");
    }

    let v_other = data["other"];

    if (v_other !== null && v_other !== undefined) {
      v_other = v_other;
    } else {
      v_other = null;
    }

    const v_data = Data.decode(data["data"]);

    if (v_data === null || v_data === undefined) {
      throw new Error("data" + ": required field");
    }

    return new Interface_One(v_name, v_other, v_data);
  }

  encode() {
    const data = {};

    data["type"] = Interface_One.TYPE;

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    if (this.other !== null && this.other !== undefined) {
      data["other"] = this.other;
    }

    if (this.data === null || this.data === undefined) {
      throw new Error("data: is a required field");
    }

    data["data"] = this.data.encode();

    return data;
  }
}

Interface_One.TYPE = "One";

export class Interface_Two {
  constructor(name, other, data) {
    this.name = name;
    this.other = other;
    this.data = data;
  }

  static decode(data) {
    const v_name = data["name"];

    if (v_name === null || v_name === undefined) {
      throw new Error("name" + ": required field");
    }

    let v_other = data["other"];

    if (v_other !== null && v_other !== undefined) {
      v_other = v_other;
    } else {
      v_other = null;
    }

    const v_data = Data.decode(data["data"]);

    if (v_data === null || v_data === undefined) {
      throw new Error("data" + ": required field");
    }

    return new Interface_Two(v_name, v_other, v_data);
  }

  encode() {
    const data = {};

    data["type"] = Interface_Two.TYPE;

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    if (this.other !== null && this.other !== undefined) {
      data["other"] = this.other;
    }

    if (this.data === null || this.data === undefined) {
      throw new Error("data: is a required field");
    }

    data["data"] = this.data.encode();

    return data;
  }
}

Interface_Two.TYPE = "Two";

export class Type {
  constructor(data, other) {
    this.data = data;
    this.other = other;
  }

  static decode(data) {
    if (data == "foo") {
      return new Type("foo", null)
    }

    if (typeof data === "string") {
      data = data
      return new Type(data, null);
    }

    const v_data = data["data"];

    if (v_data === null || v_data === undefined) {
      throw new Error("data" + ": required field");
    }

    let v_other = data["other"];

    if (v_other !== null && v_other !== undefined) {
      v_other = v_other;
    } else {
      v_other = null;
    }

    return new Type(v_data, v_other);
  }

  encode() {
    const data = {};

    if (this.data === null || this.data === undefined) {
      throw new Error("data: is a required field");
    }

    data["data"] = this.data;

    if (this.other !== null && this.other !== undefined) {
      data["other"] = this.other;
    }

    return data;
  }
}
