import * as c from "heroic/common.js";

class Sampling {
  constructor(unit, size, extent) {
    this.unit = unit;
    this.size = size;
    this.extent = extent;
  }

  static decode(data) {
    let unit = data["unit"];

    if (unit !== null && unit !== undefined) {
      unit = TimeUnit.decode(unit);
    } else {
      unit = null;
    }

    const size = data["size"];

    let extent = data["extent"];

    if (extent !== null && extent !== undefined) {
      extent = extent;
    } else {
      extent = null;
    }

    return new Sampling(unit, size, extent);
  }

  encode() {
    const data = {};

    if (this.unit !== null && this.unit !== undefined) {
      data["unit"] = this.unit.encode();
    }

    if (this.size === null || this.size === undefined) {
      throw new Error("size: is a required field");
    }

    data["size"] = this.size;

    if (this.extent !== null && this.extent !== undefined) {
      data["extent"] = this.extent;
    }

    return data;
  }
}

class SI {
  constructor(ordinal, name) {
    this.ordinal = ordinal;
    this.name = name;
  }

  encode() {
    return this.name;
  }
  static decode(data) {
    for (let i = 0, l = SI.values.length; i < l; i++) {
      const member = SI.values[i]



      if (member.name === data) {
        return member;
      }
    }

    throw new Error("no matching value");
  }
}

SI.NANO = new SI(3, "NANO");
SI.MICRO = new SI(2, "MICRO");
SI.MILLI = new SI(10, "MILLI");

SI.values = [SI.NANO, SI.MICRO, SI.MILLI];

class TimeUnit {
  constructor(ordinal, name, _name, number) {
    this.ordinal = ordinal;
    this.name = name;
    this._name = _name;
    this.number = number;
  }

  encode() {
    return this.number;
  }
  static decode(data) {
    for (let i = 0, l = TimeUnit.values.length; i < l; i++) {
      const member = TimeUnit.values[i]



      if (member.number === data) {
        return member;
      }
    }

    throw new Error("no matching value");
  }
}

TimeUnit.SECONDS = new TimeUnit(0, "SECONDS", "seconds", 1000);
TimeUnit.MINUTES = new TimeUnit(1, "MINUTES", "minutes", 60000);

TimeUnit.values = [TimeUnit.SECONDS, TimeUnit.MINUTES];

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

class Event {
  constructor(timestamp, payload) {
    this.timestamp = timestamp;
    this.payload = payload;
  }

  static decode(data) {
    const timestamp = data[0];

    const payload = data[1];

    return new Event(timestamp, payload);
  }

  encode() {
    if (this.timestamp === null || this.timestamp === undefined) {
      throw new Error("timestamp: is a required field");
    }

    if (this.payload === null || this.payload === undefined) {
      throw new Error("payload: is a required field");
    }

    return [this.timestamp, this.payload];
  }
}

class Samples {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "events") {
      return Events.decode(data);
    }

    if (f_type === "points") {
      return Points.decode(data);
    }

    throw new Error("bad type");
  }
}

class Events {
  constructor(name, data) {
    this.name = name;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    const data = data["data"].map(function(v) { Event.decode(v); });

    return new Events(name, data);
  }

  encode() {
    const data = {};

    data["type"] = Events.TYPE;

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    if (this.data === null || this.data === undefined) {
      throw new Error("data: is a required field");
    }

    data["data"] = this.data.map(function(v) { return v.encode(); });

    return data;
  }
}

Events.TYPE = "Events";

class Points {
  constructor(name, data) {
    this.name = name;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    const data = data["data"].map(function(v) { Point.decode(v); });

    return new Points(name, data);
  }

  encode() {
    const data = {};

    data["type"] = Points.TYPE;

    if (this.name === null || this.name === undefined) {
      throw new Error("name: is a required field");
    }

    data["name"] = this.name;

    if (this.data === null || this.data === undefined) {
      throw new Error("data: is a required field");
    }

    data["data"] = this.data.map(function(v) { return v.encode(); });

    return data;
  }
}

Points.TYPE = "Points";

class Query {
  constructor(query, aggregation, date, parameters) {
    this.query = query;
    this.aggregation = aggregation;
    this.date = date;
    this.parameters = parameters;
  }

  static decode(data) {
    if (typeof data === "string") {
      query = data
      return new Query(query, null, null, null);
    }

    let query = data["query"];

    if (query !== null && query !== undefined) {
      query = query;
    } else {
      query = null;
    }

    let aggregation = data["aggregation"];

    if (aggregation !== null && aggregation !== undefined) {
      aggregation = Aggregation.decode(aggregation);
    } else {
      aggregation = null;
    }

    let date = data["date"];

    if (date !== null && date !== undefined) {
      date = c.Date.decode(date);
    } else {
      date = null;
    }

    let parameters = data["parameters"];

    if (parameters !== null && parameters !== undefined) {
      parameters = parameters;
    } else {
      parameters = null;
    }

    return new Query(query, aggregation, date, parameters);
  }

  encode() {
    const data = {};

    if (this.query !== null && this.query !== undefined) {
      data["query"] = this.query;
    }

    if (this.aggregation !== null && this.aggregation !== undefined) {
      data["aggregation"] = this.aggregation.encode();
    }

    if (this.date !== null && this.date !== undefined) {
      data["date"] = this.date.encode();
    }

    if (this.parameters !== null && this.parameters !== undefined) {
      data["parameters"] = this.parameters;
    }

    return data;
  }
}

class Duration {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "absolute") {
      return Absolute.decode(data);
    }

    throw new Error("bad type");
  }
}

class Absolute {
  constructor(start, end) {
    this.start = start;
    this.end = end;
  }

  static decode(data) {
    const start = data["start"];

    const end = data["end"];

    return new Absolute(start, end);
  }

  encode() {
    const data = {};

    data["type"] = Absolute.TYPE;

    if (this.start === null || this.start === undefined) {
      throw new Error("start: is a required field");
    }

    data["start"] = this.start;

    if (this.end === null || this.end === undefined) {
      throw new Error("end: is a required field");
    }

    data["end"] = this.end;

    return data;
  }
}

Absolute.TYPE = "Absolute";

class Aggregation {
  static decode(data) {
    const f_type = data["type"]

    if (f_type === "average") {
      return Average.decode(data);
    }

    if (f_type === "chain") {
      return Chain.decode(data);
    }

    if (f_type === "sum") {
      return Sum.decode(data);
    }

    throw new Error("bad type");
  }
}

class Average {
  constructor(sampling, size, extent) {
    this.sampling = sampling;
    this.size = size;
    this.extent = extent;
  }

  static decode(data) {
    let sampling = data["sampling"];

    if (sampling !== null && sampling !== undefined) {
      sampling = Sampling.decode(sampling);
    } else {
      sampling = null;
    }

    let size = data["size"];

    if (size !== null && size !== undefined) {
      size = Duration.decode(size);
    } else {
      size = null;
    }

    let extent = data["extent"];

    if (extent !== null && extent !== undefined) {
      extent = Duration.decode(extent);
    } else {
      extent = null;
    }

    return new Average(sampling, size, extent);
  }

  encode() {
    const data = {};

    data["type"] = Average.TYPE;

    if (this.sampling !== null && this.sampling !== undefined) {
      data["sampling"] = this.sampling.encode();
    }

    if (this.size !== null && this.size !== undefined) {
      data["size"] = this.size.encode();
    }

    if (this.extent !== null && this.extent !== undefined) {
      data["extent"] = this.extent.encode();
    }

    return data;
  }
}

Average.TYPE = "Average";

class Chain {
  constructor(chain) {
    this.chain = chain;
  }

  static decode(data) {
    const chain = data["chain"].map(function(v) { Aggregation.decode(v); });

    return new Chain(chain);
  }

  encode() {
    const data = {};

    data["type"] = Chain.TYPE;

    if (this.chain === null || this.chain === undefined) {
      throw new Error("chain: is a required field");
    }

    data["chain"] = this.chain.map(function(v) { return v.encode(); });

    return data;
  }
}

Chain.TYPE = "Chain";

class Sum {
  constructor(sampling, size, extent) {
    this.sampling = sampling;
    this.size = size;
    this.extent = extent;
  }

  static decode(data) {
    let sampling = data["sampling"];

    if (sampling !== null && sampling !== undefined) {
      sampling = Sampling.decode(sampling);
    } else {
      sampling = null;
    }

    let size = data["size"];

    if (size !== null && size !== undefined) {
      size = Duration.decode(size);
    } else {
      size = null;
    }

    let extent = data["extent"];

    if (extent !== null && extent !== undefined) {
      extent = Duration.decode(extent);
    } else {
      extent = null;
    }

    return new Sum(sampling, size, extent);
  }

  encode() {
    const data = {};

    data["type"] = Sum.TYPE;

    if (this.sampling !== null && this.sampling !== undefined) {
      data["sampling"] = this.sampling.encode();
    }

    if (this.size !== null && this.size !== undefined) {
      data["size"] = this.size.encode();
    }

    if (this.extent !== null && this.extent !== undefined) {
      data["extent"] = this.extent.encode();
    }

    return data;
  }
}

Sum.TYPE = "Sum";

class ComplexEnum {
  constructor(ordinal, name, si, other, samples) {
    this.ordinal = ordinal;
    this.name = name;
    this.si = si;
    this.other = other;
    this.samples = samples;
  }

  encode() {
    return this.ordinal;
  }
  static decode(data) {
    for (let i = 0, l = ComplexEnum.values.length; i < l; i++) {
      const member = ComplexEnum.values[i]



      if (member.ordinal === data) {
        return member;
      }
    }

    throw new Error("no matching value");
  }
}

ComplexEnum.FIRST = new ComplexEnum(0, "FIRST", new Sampling(null, 42, null), SI.NANO, new Samples.Points("points", []));
ComplexEnum.SECOND = new ComplexEnum(1, "SECOND", new Sampling(null, 9, null), SI.MILLI, new Samples.Points("b", []));

ComplexEnum.values = [ComplexEnum.FIRST, ComplexEnum.SECOND];

class Complex21 {
  constructor(ordinal, name, point) {
    this.ordinal = ordinal;
    this.name = name;
    this.point = point;
  }

  encode() {
    return this.ordinal;
  }
  static decode(data) {
    for (let i = 0, l = Complex21.values.length; i < l; i++) {
      const member = Complex21.values[i]



      if (member.ordinal === data) {
        return member;
      }
    }

    throw new Error("no matching value");
  }
}

Complex21.FIRST = new Complex21(0, "FIRST", new Point(123, 42.1));
Complex21.SECOND = new Complex21(1, "SECOND", new Point(123, 1234.12));

Complex21.values = [Complex21.FIRST, Complex21.SECOND];
