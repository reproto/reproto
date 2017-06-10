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
    if (typeof data === "string") {
      name = data
      return new Samples_Points(name, []);
    }

    const f_type = data["type"]

    if (f_type === "events") {
      return Samples_Events.decode(data);
    }

    if (f_type === "points") {
      return Samples_Points.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

class Samples_Events {
  constructor(name, data) {
    this.name = name;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    const data = data["data"].map(function(v) { Event.decode(v); });

    return new Samples_Events(name, data);
  }

  encode() {
    const data = {};

    data["type"] = Samples_Events.TYPE;

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

Samples_Events.TYPE = "Events";

class Samples_Points {
  constructor(name, data) {
    this.name = name;
    this.data = data;
  }

  static decode(data) {
    const name = data["name"];

    const data = data["data"].map(function(v) { Point.decode(v); });

    return new Samples_Points(name, data);
  }

  encode() {
    const data = {};

    data["type"] = Samples_Points.TYPE;

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

Samples_Points.TYPE = "Points";

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
      return Duration_Absolute.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

class Duration_Absolute {
  constructor(start, end) {
    this.start = start;
    this.end = end;
  }

  static decode(data) {
    const start = data["start"];

    const end = data["end"];

    return new Duration_Absolute(start, end);
  }

  encode() {
    const data = {};

    data["type"] = Duration_Absolute.TYPE;

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

Duration_Absolute.TYPE = "Absolute";

class Aggregation {
  static decode(data) {
    if (data.constructor === Array) {
      chain = data.map(function(v) { Aggregation.decode(v); })
      return new Aggregation_Chain(chain);
    }

    const f_type = data["type"]

    if (f_type === "average") {
      return Aggregation_Average.decode(data);
    }

    if (f_type === "chain") {
      return Aggregation_Chain.decode(data);
    }

    if (f_type === "sum") {
      return Aggregation_Sum.decode(data);
    }

    throw new Error("bad type: " + f_type);
  }
}

class Aggregation_Average {
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

    return new Aggregation_Average(sampling, size, extent);
  }

  encode() {
    const data = {};

    data["type"] = Aggregation_Average.TYPE;

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

Aggregation_Average.TYPE = "Average";

class Aggregation_Chain {
  constructor(chain) {
    this.chain = chain;
  }

  static decode(data) {
    const chain = data["chain"].map(function(v) { Aggregation.decode(v); });

    return new Aggregation_Chain(chain);
  }

  encode() {
    const data = {};

    data["type"] = Aggregation_Chain.TYPE;

    if (this.chain === null || this.chain === undefined) {
      throw new Error("chain: is a required field");
    }

    data["chain"] = this.chain.map(function(v) { return v.encode(); });

    return data;
  }
}

Aggregation_Chain.TYPE = "Chain";

class Aggregation_Sum {
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

    return new Aggregation_Sum(sampling, size, extent);
  }

  encode() {
    const data = {};

    data["type"] = Aggregation_Sum.TYPE;

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

Aggregation_Sum.TYPE = "Sum";

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

ComplexEnum.FIRST = new ComplexEnum(0, "FIRST", new Sampling(null, 42, null), SI_NANO, new Samples_Points("points", []));
ComplexEnum.SECOND = new ComplexEnum(1, "SECOND", new Sampling(null, 9, null), SI_MILLI, new Samples_Points("b", []));

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
