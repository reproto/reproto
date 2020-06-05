class Entry:
  def __init__(self, _tuple1, _tuple2):
    self._tuple1 = _tuple1
    self._tuple2 = _tuple2

  @property
  def tuple1(self):
    return self._tuple1

  @property
  def tuple2(self):
    return self._tuple2

  @staticmethod
  def decode(data):
    f_tuple1 = None

    if "tuple1" in data:
      f_tuple1 = data["tuple1"]

      if f_tuple1 is not None:
        f_tuple1 = Tuple1.decode(f_tuple1)

    f_tuple2 = None

    if "tuple2" in data:
      f_tuple2 = data["tuple2"]

      if f_tuple2 is not None:
        f_tuple2 = Tuple2.decode(f_tuple2)

    return Entry(f_tuple1, f_tuple2)

  def encode(self):
    data = dict()

    if self._tuple1 is not None:
      data["tuple1"] = self._tuple1.encode()

    if self._tuple2 is not None:
      data["tuple2"] = self._tuple2.encode()

    return data

  def __repr__(self):
    return "<Entry tuple1:{!r}, tuple2:{!r}>".format(self._tuple1, self._tuple2)

class Tuple1:
  def __init__(self, _a, _b):
    self._a = _a
    self._b = _b

  @property
  def a(self):
    return self._a

  @property
  def b(self):
    return self._b

  @staticmethod
  def decode(data):
    f_a = data[0]

    if not isinstance(f_a, unicode):
      raise Exception("not a string")

    f_b = data[1]

    if not isinstance(f_b, int):
      raise Exception("not an integer")

    return Tuple1(f_a, f_b)

  def encode(self):
    if self._a is None:
      raise Exception("a: is a required field")

    if self._b is None:
      raise Exception("b: is a required field")

    return (self._a, self._b)

  def __repr__(self):
    return "<Tuple1 a:{!r}, b:{!r}>".format(self._a, self._b)

class Tuple2:
  def __init__(self, _a, _b):
    self._a = _a
    self._b = _b

  @property
  def a(self):
    return self._a

  @property
  def b(self):
    return self._b

  @staticmethod
  def decode(data):
    f_a = data[0]

    if not isinstance(f_a, unicode):
      raise Exception("not a string")

    f_b = data[1]

    f_b = Other.decode(f_b)

    return Tuple2(f_a, f_b)

  def encode(self):
    if self._a is None:
      raise Exception("a: is a required field")

    if self._b is None:
      raise Exception("b: is a required field")

    return (self._a, self._b.encode())

  def __repr__(self):
    return "<Tuple2 a:{!r}, b:{!r}>".format(self._a, self._b)

class Other:
  def __init__(self, _a):
    self._a = _a

  @property
  def a(self):
    return self._a

  @staticmethod
  def decode(data):
    f_a = data["a"]

    if not isinstance(f_a, unicode):
      raise Exception("not a string")

    return Other(f_a)

  def encode(self):
    data = dict()

    if self._a is None:
      raise Exception("a: is a required field")

    data["a"] = self._a

    return data

  def __repr__(self):
    return "<Other a:{!r}>".format(self._a)
