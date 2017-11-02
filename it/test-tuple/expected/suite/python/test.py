class Entry:
  def __init__(self, tuple1, tuple2):
    self.tuple1 = tuple1
    self.tuple2 = tuple2

  @staticmethod
  def decode(data):
    if "tuple1" in data:
      f_tuple1 = data["tuple1"]

      if f_tuple1 is not None:
        f_tuple1 = Tuple1.decode(f_tuple1)
    else:
      f_tuple1 = None

    if "tuple2" in data:
      f_tuple2 = data["tuple2"]

      if f_tuple2 is not None:
        f_tuple2 = Tuple2.decode(f_tuple2)
    else:
      f_tuple2 = None

    return Entry(f_tuple1, f_tuple2)

  def encode(self):
    data = dict()

    if self.tuple1 is not None:
      data["tuple1"] = self.tuple1.encode()

    if self.tuple2 is not None:
      data["tuple2"] = self.tuple2.encode()

    return data

  def __repr__(self):
    return "<Entry tuple1: {!r}, tuple2: {!r}>".format(self.tuple1, self.tuple2)

class Tuple1:
  def __init__(self, a, b):
    self.a = a
    self.b = b

  @staticmethod
  def decode(data):
    f_a = data[0]

    f_b = data[1]

    return Tuple1(f_a, f_b)

  def encode(self):
    if self.a is None:
      raise Exception("a: is a required field")

    if self.b is None:
      raise Exception("b: is a required field")

    return (self.a, self.b)

  def __repr__(self):
    return "<Tuple1 a: {!r}, b: {!r}>".format(self.a, self.b)

class Tuple2:
  def __init__(self, a, b):
    self.a = a
    self.b = b

  @staticmethod
  def decode(data):
    f_a = data[0]

    f_b = Other.decode(data[1])

    return Tuple2(f_a, f_b)

  def encode(self):
    if self.a is None:
      raise Exception("a: is a required field")

    if self.b is None:
      raise Exception("b: is a required field")

    return (self.a, self.b.encode())

  def __repr__(self):
    return "<Tuple2 a: {!r}, b: {!r}>".format(self.a, self.b)

class Other:
  def __init__(self, a):
    self.a = a

  @staticmethod
  def decode(data):
    f_a = data["a"]

    return Other(f_a)

  def encode(self):
    data = dict()

    if self.a is None:
      raise Exception("a: is a required field")

    data["a"] = self.a

    return data

  def __repr__(self):
    return "<Other a: {!r}>".format(self.a)
