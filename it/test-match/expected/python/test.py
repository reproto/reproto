import numbers

class Entry:
  def __init__(self, data, point, interface_field, type_field):
    self.data = data
    self.point = point
    self.interface_field = interface_field
    self.type_field = type_field

  @staticmethod
  def decode(data):
    if "data" in data:
      f_data = data["data"]

      if f_data is not None:
        f_data = Data.decode(f_data)
    else:
      f_data = None

    if "point" in data:
      f_point = data["point"]

      if f_point is not None:
        f_point = Point.decode(f_point)
    else:
      f_point = None

    if "interface" in data:
      f_interface_field = data["interface"]

      if f_interface_field is not None:
        f_interface_field = Interface.decode(f_interface_field)
    else:
      f_interface_field = None

    if "type" in data:
      f_type_field = data["type"]

      if f_type_field is not None:
        f_type_field = Type.decode(f_type_field)
    else:
      f_type_field = None

    return Entry(f_data, f_point, f_interface_field, f_type_field)

  def encode(self):
    data = dict()

    if self.data is not None:
      data["data"] = self.data.encode()

    if self.point is not None:
      data["point"] = self.point.encode()

    if self.interface_field is not None:
      data["interface"] = self.interface_field.encode()

    if self.type_field is not None:
      data["type"] = self.type_field.encode()

    return data

  def __repr__(self):
    return "<Entry data: {!r}, point: {!r}, interface_field: {!r}, type_field: {!r}>".format(self.data, self.point, self.interface_field, self.type_field)

class Data:
  def __init__(self, name):
    self.name = name

  @staticmethod
  def decode(data):
    f_name = data["name"]

    return Data(f_name)

  def encode(self):
    data = dict()

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    return data

  def __repr__(self):
    return "<Data name: {!r}>".format(self.name)

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
      raise Exception("TS: is a required field")

    if self.value is None:
      raise Exception("value: is a required field")

    return (self.timestamp, self.value)

  def __repr__(self):
    return "<Point timestamp: {!r}, value: {!r}>".format(self.timestamp, self.value)

class Interface:
  @staticmethod
  def decode(data):
    if isinstance(data, str):
      name = data
      return Interface_One(name, None, Data("data"))

    f_type = data["type"]

    if f_type == "one":
      return Interface_One.decode(data)

    if f_type == "two":
      return Interface_Two.decode(data)

    raise Exception("bad type" + f_type)

class Interface_One(Interface):
  TYPE = "one"

  def __init__(self, name, other, data):
    self.name = name
    self.other = other
    self.data = data

  @staticmethod
  def decode(data):
    f_name = data["name"]

    if "other" in data:
      f_other = data["other"]

      if f_other is not None:
        f_other = f_other
    else:
      f_other = None

    f_data = Data.decode(data["data"])

    return Interface_One(f_name, f_other, f_data)

  def encode(self):
    data = dict()

    data["type"] = "one"

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    if self.other is not None:
      data["other"] = self.other

    if self.data is None:
      raise Exception("data: is a required field")

    data["data"] = self.data.encode()

    return data

  def __repr__(self):
    return "<Interface_One name: {!r}, other: {!r}, data: {!r}>".format(self.name, self.other, self.data)

class Interface_Two(Interface):
  TYPE = "two"

  def __init__(self, name, other, data):
    self.name = name
    self.other = other
    self.data = data

  @staticmethod
  def decode(data):
    f_name = data["name"]

    if "other" in data:
      f_other = data["other"]

      if f_other is not None:
        f_other = f_other
    else:
      f_other = None

    f_data = Data.decode(data["data"])

    return Interface_Two(f_name, f_other, f_data)

  def encode(self):
    data = dict()

    data["type"] = "two"

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    if self.other is not None:
      data["other"] = self.other

    if self.data is None:
      raise Exception("data: is a required field")

    data["data"] = self.data.encode()

    return data

  def __repr__(self):
    return "<Interface_Two name: {!r}, other: {!r}, data: {!r}>".format(self.name, self.other, self.data)

class Type:
  def __init__(self, data, other):
    self.data = data
    self.other = other

  @staticmethod
  def decode(data):
    if data == "foo":
      return Type("foo", None)

    if isinstance(data, str):
      data = data
      return Type(data, None)

    f_data = data["data"]

    if "other" in data:
      f_other = data["other"]

      if f_other is not None:
        f_other = f_other
    else:
      f_other = None

    return Type(f_data, f_other)

  def encode(self):
    data = dict()

    if self.data is None:
      raise Exception("data: is a required field")

    data["data"] = self.data

    if self.other is not None:
      data["other"] = self.other

    return data

  def __repr__(self):
    return "<Type data: {!r}, other: {!r}>".format(self.data, self.other)
