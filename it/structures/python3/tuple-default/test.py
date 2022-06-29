class Entry:
  def __init__(self, tuple1, tuple2):
    self.__tuple1 = tuple1
    self.__tuple2 = tuple2

  @property
  def tuple1(self):
    return self.__tuple1

  @tuple1.setter
  def tuple1(self, tuple1):
    self.__tuple1 = tuple1

  @property
  def tuple2(self):
    return self.__tuple2

  @tuple2.setter
  def tuple2(self, tuple2):
    self.__tuple2 = tuple2

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

    if self.tuple1 is not None:
      data["tuple1"] = self.tuple1.encode()

    if self.tuple2 is not None:
      data["tuple2"] = self.tuple2.encode()

    return data

  def __repr__(self):
    return "<Entry tuple1:{!r}, tuple2:{!r}>".format(self.tuple1, self.tuple2)

class Tuple1:
  def __init__(self, a, b):
    self.__a = a
    self.__b = b

  @property
  def a(self):
    return self.__a

  @a.setter
  def a(self, a):
    self.__a = a

  @property
  def b(self):
    return self.__b

  @b.setter
  def b(self, b):
    self.__b = b

  @staticmethod
  def decode(data):
    f_a = data[0]

    if not isinstance(f_a, str):
      raise Exception("not a string")

    f_b = data[1]

    if not isinstance(f_b, int):
      raise Exception("not an integer")

    return Tuple1(f_a, f_b)

  def encode(self):
    if self.a is None:
      raise Exception("missing required field: a")

    a = self.a

    if self.b is None:
      raise Exception("missing required field: b")

    b = self.b

    return (a, b)

  def __repr__(self):
    return "<Tuple1 a:{!r}, b:{!r}>".format(self.a, self.b)

class Tuple2:
  def __init__(self, a, b):
    self.__a = a
    self.__b = b

  @property
  def a(self):
    return self.__a

  @a.setter
  def a(self, a):
    self.__a = a

  @property
  def b(self):
    return self.__b

  @b.setter
  def b(self, b):
    self.__b = b

  @staticmethod
  def decode(data):
    f_a = data[0]

    if not isinstance(f_a, str):
      raise Exception("not a string")

    f_b = data[1]

    f_b = Other.decode(f_b)

    return Tuple2(f_a, f_b)

  def encode(self):
    if self.a is None:
      raise Exception("missing required field: a")

    a = self.a

    if self.b is None:
      raise Exception("missing required field: b")

    b = self.b.encode()

    return (a, b)

  def __repr__(self):
    return "<Tuple2 a:{!r}, b:{!r}>".format(self.a, self.b)

class Other:
  def __init__(self, a):
    self.__a = a

  @property
  def a(self):
    return self.__a

  @a.setter
  def a(self, a):
    self.__a = a

  @staticmethod
  def decode(data):
    f_a = data["a"]

    if not isinstance(f_a, str):
      raise Exception("not a string")

    return Other(f_a)

  def encode(self):
    data = dict()

    if self.a is None:
      raise Exception("missing required field: a")

    data["a"] = self.a

    return data

  def __repr__(self):
    return "<Other a:{!r}>".format(self.a)
