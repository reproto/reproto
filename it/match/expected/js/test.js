class Data {
  constructor(name) {
    this.name = name;
  }

  static decode(data) {
    const name = data["name"];

    return new Data(name);
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

class Point {
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

    if (data.constructor === Object) {
      p = Point.decode(data)
      return p;
    }

    const timestamp = data[0];

    const value = data[1];

    return new Point(timestamp, value);
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

class Interface {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "one") {
      return One.decode(data);
    }

    if (f_type === "two") {
      return Two.decode(data);
    }

    throw new Error("bad type");
  }
}

class One {
  constructor(name, other, data) {
    this.name = name;
    this.other = other;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    let other = data["other"];

    if (other !== null && other !== undefined) {
      other = other;
    } else {
      other = null;
    }

    const data = Data.decode(data["data"]);

    return new One(name, other, data);
  }

  encode() {
    const data = {};

    data["type"] = One.TYPE;

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

One.TYPE = "One";

class Two {
  constructor(name, other, data) {
    this.name = name;
    this.other = other;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    let other = data["other"];

    if (other !== null && other !== undefined) {
      other = other;
    } else {
      other = null;
    }

    const data = Data.decode(data["data"]);

    return new Two(name, other, data);
  }

  encode() {
    const data = {};

    data["type"] = Two.TYPE;

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

Two.TYPE = "Two";

class Type {
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

    const data = data["data"];

    let other = data["other"];

    if (other !== null && other !== undefined) {
      other = other;
    } else {
      other = null;
    }

    return new Type(data, other);
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
