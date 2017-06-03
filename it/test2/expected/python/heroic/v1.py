import enum
import heroic.common as c
import numbers

class Sampling:
  def __init__(self, unit, size, extent):
    self.unit = unit
    self.size = size
    self.extent = extent

  @staticmethod
  def decode(data):
    if "unit" in data:
      f_unit = data["unit"]

      if f_unit is not None:
        f_unit = TimeUnit.decode(f_unit)
    else:
      f_unit = None

    f_size = data["size"]

    if "extent" in data:
      f_extent = data["extent"]

      if f_extent is not None:
        f_extent = f_extent
    else:
      f_extent = None

    return Sampling(f_unit, f_size, f_extent)

  def encode(self):
    data = dict()

    if self.unit is not None:
      data["unit"] = self.unit.encode()

    if self.size is None:
      raise Exception("size: is a required field")

    data["size"] = self.size

    if self.extent is not None:
      data["extent"] = self.extent

    return data

class SI:
  pass

class TimeUnit:
  def __init__(self, _name, number):
    self._name = _name
    self.number = number

  def encode(self):
    return self.number

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value.number == data:
        return value

    raise Exception("data does not match enum")

class Point:
  def __init__(self, timestamp, value):
    self.timestamp = timestamp
    self.value = value

  @staticmethod
  def decode(data):
    if data == 42:
      return Point(42, 41.2)

    if isinstance(data, numbers.Number):
      n = data
      return Point(n, 42)

    if isinstance(data, dict):
      p = Point.decode(data)
      return p

    f_timestamp = data[0]

    f_value = data[1]

    return Point(f_timestamp, f_value)

  def encode(self):
    if self.timestamp is None:
      raise Exception("timestamp: is a required field")

    if self.value is None:
      raise Exception("value: is a required field")

    return (self.timestamp, self.value)

class Event:
  def __init__(self, timestamp, payload):
    self.timestamp = timestamp
    self.payload = payload

  @staticmethod
  def decode(data):
    f_timestamp = data[0]

    f_payload = data[1]

    return Event(f_timestamp, f_payload)

  def encode(self):
    if self.timestamp is None:
      raise Exception("timestamp: is a required field")

    if self.payload is None:
      raise Exception("payload: is a required field")

    return (self.timestamp, self.payload)

class Samples:
  @staticmethod
  def decode(data):
    if isinstance(data, basestring):
      name = data
      return Samples_Points(name, [])

    f_type = data["type"]

    if f_type == "events":
      return Samples_Events.decode(data)

    if f_type == "points":
      return Samples_Points.decode(data)

    raise Exception("bad type" + f_type)

class Samples_Events(Samples):
  TYPE = "events"

  def __init__(self, name, data):
    self.name = name
    self.data = data

  @staticmethod
  def decode(data):
    if isinstance(data, basestring):
      name = data
      return Samples_Points(name, [])

    f_name = data["name"]

    f_data = map(lambda v: Event.decode(v), data["data"])

    return Samples_Events(f_name, f_data)

  def encode(self):
    data = dict()

    data["type"] = "events"

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    if self.data is None:
      raise Exception("data: is a required field")

    data["data"] = map(lambda v: v.encode(), self.data)

    return data

class Samples_Points(Samples):
  TYPE = "points"

  def __init__(self, name, data):
    self.name = name
    self.data = data

  @staticmethod
  def decode(data):
    if isinstance(data, basestring):
      name = data
      return Samples_Points(name, [])

    f_name = data["name"]

    f_data = map(lambda v: Point.decode(v), data["data"])

    return Samples_Points(f_name, f_data)

  def encode(self):
    data = dict()

    data["type"] = "points"

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    if self.data is None:
      raise Exception("data: is a required field")

    data["data"] = map(lambda v: v.encode(), self.data)

    return data

class Query:
  def __init__(self, query, aggregation, date, parameters):
    self.query = query
    self.aggregation = aggregation
    self.date = date
    self.parameters = parameters

  @staticmethod
  def decode(data):
    if "query" in data:
      f_query = data["query"]

      if f_query is not None:
        f_query = f_query
    else:
      f_query = None

    if "aggregation" in data:
      f_aggregation = data["aggregation"]

      if f_aggregation is not None:
        f_aggregation = Aggregation.decode(f_aggregation)
    else:
      f_aggregation = None

    if "date" in data:
      f_date = data["date"]

      if f_date is not None:
        f_date = c.Date.decode(f_date)
    else:
      f_date = None

    if "parameters" in data:
      f_parameters = data["parameters"]

      if f_parameters is not None:
        f_parameters = dict(f_parameters.items().map(lambda t: (t[0], t[1])))
    else:
      f_parameters = None

    return Query(f_query, f_aggregation, f_date, f_parameters)

  def encode(self):
    data = dict()

    if self.query is not None:
      data["query"] = self.query

    if self.aggregation is not None:
      data["aggregation"] = self.aggregation.encode()

    if self.date is not None:
      data["date"] = self.date.encode()

    if self.parameters is not None:
      data["parameters"] = self.parameters

    return data

class Duration:
  @staticmethod
  def decode(data):
    f_type = data["type"]

    if f_type == "absolute":
      return Duration_Absolute.decode(data)

    raise Exception("bad type" + f_type)

class Duration_Absolute(Duration):
  TYPE = "absolute"

  def __init__(self, start, end):
    self.start = start
    self.end = end

  @staticmethod
  def decode(data):
    f_start = data["start"]

    f_end = data["end"]

    return Duration_Absolute(f_start, f_end)

  def encode(self):
    data = dict()

    data["type"] = "absolute"

    if self.start is None:
      raise Exception("start: is a required field")

    data["start"] = self.start

    if self.end is None:
      raise Exception("end: is a required field")

    data["end"] = self.end

    return data

class Aggregation:
  @staticmethod
  def decode(data):
    if isinstance(data, list):
      chain = map(lambda v: Aggregation.decode(v), data)
      return Aggregation_Chain(chain)

    f_type = data["type"]

    if f_type == "average":
      return Aggregation_Average.decode(data)

    if f_type == "chain":
      return Aggregation_Chain.decode(data)

    if f_type == "sum":
      return Aggregation_Sum.decode(data)

    raise Exception("bad type" + f_type)

class Aggregation_Average(Aggregation):
  TYPE = "average"

  def __init__(self, sampling, size, extent):
    self.sampling = sampling
    self.size = size
    self.extent = extent

  @staticmethod
  def decode(data):
    if isinstance(data, list):
      chain = map(lambda v: Aggregation.decode(v), data)
      return Aggregation_Chain(chain)

    if "sampling" in data:
      f_sampling = data["sampling"]

      if f_sampling is not None:
        f_sampling = Sampling.decode(f_sampling)
    else:
      f_sampling = None

    if "size" in data:
      f_size = data["size"]

      if f_size is not None:
        f_size = Duration.decode(f_size)
    else:
      f_size = None

    if "extent" in data:
      f_extent = data["extent"]

      if f_extent is not None:
        f_extent = Duration.decode(f_extent)
    else:
      f_extent = None

    return Aggregation_Average(f_sampling, f_size, f_extent)

  def encode(self):
    data = dict()

    data["type"] = "average"

    if self.sampling is not None:
      data["sampling"] = self.sampling.encode()

    if self.size is not None:
      data["size"] = self.size.encode()

    if self.extent is not None:
      data["extent"] = self.extent.encode()

    return data

class Aggregation_Chain(Aggregation):
  TYPE = "chain"

  def __init__(self, chain):
    self.chain = chain

  @staticmethod
  def decode(data):
    if isinstance(data, list):
      chain = map(lambda v: Aggregation.decode(v), data)
      return Aggregation_Chain(chain)

    f_chain = map(lambda v: Aggregation.decode(v), data["chain"])

    return Aggregation_Chain(f_chain)

  def encode(self):
    data = dict()

    data["type"] = "chain"

    if self.chain is None:
      raise Exception("chain: is a required field")

    data["chain"] = map(lambda v: v.encode(), self.chain)

    return data

class Aggregation_Sum(Aggregation):
  TYPE = "sum"

  def __init__(self, sampling, size, extent):
    self.sampling = sampling
    self.size = size
    self.extent = extent

  @staticmethod
  def decode(data):
    if isinstance(data, list):
      chain = map(lambda v: Aggregation.decode(v), data)
      return Aggregation_Chain(chain)

    if "sampling" in data:
      f_sampling = data["sampling"]

      if f_sampling is not None:
        f_sampling = Sampling.decode(f_sampling)
    else:
      f_sampling = None

    if "size" in data:
      f_size = data["size"]

      if f_size is not None:
        f_size = Duration.decode(f_size)
    else:
      f_size = None

    if "extent" in data:
      f_extent = data["extent"]

      if f_extent is not None:
        f_extent = Duration.decode(f_extent)
    else:
      f_extent = None

    return Aggregation_Sum(f_sampling, f_size, f_extent)

  def encode(self):
    data = dict()

    data["type"] = "sum"

    if self.sampling is not None:
      data["sampling"] = self.sampling.encode()

    if self.size is not None:
      data["size"] = self.size.encode()

    if self.extent is not None:
      data["extent"] = self.extent.encode()

    return data

class ComplexEnum:
  def __init__(self, si, other, samples):
    self.si = si
    self.other = other
    self.samples = samples

class Complex21:
  def __init__(self, point):
    self.point = point

SI = enum.Enum("SI", [("NANO", 3), ("MICRO", 2), ("MILLI", 10)], type=SI)

TimeUnit = enum.Enum("TimeUnit", [("SECONDS", ("seconds", 1000)), ("MINUTES", ("minutes", 60000))], type=TimeUnit)

ComplexEnum = enum.Enum("ComplexEnum", [("FIRST", (Sampling(None, 42, None), SI.NANO, Samples_Points("points", []))), ("SECOND", (Sampling(None, 9, None), SI.MILLI, Samples_Points("b", [])))], type=ComplexEnum)

Complex21 = enum.Enum("Complex21", [("FIRST", (Point(123, 42.1))), ("SECOND", (Point(123, 1234.12)))], type=Complex21)
