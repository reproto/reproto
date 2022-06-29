class Entry:
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
    f_a = None

    if "a" in data:
      f_a = data["a"]

      if f_a is not None:
        f_a = A.decode(f_a)

    f_b = None

    if "b" in data:
      f_b = data["b"]

      if f_b is not None:
        f_b = A_B.decode(f_b)

    return Entry(f_a, f_b)

  def encode(self):
    data = dict()

    if self.a is not None:
      data["a"] = self.a.encode()

    if self.b is not None:
      data["b"] = self.b.encode()

    return data

  def __repr__(self):
    return "<Entry a:{!r}, b:{!r}>".format(self.a, self.b)

class A:
  def __init__(self, b):
    self.__b = b

  @property
  def b(self):
    return self.__b

  @b.setter
  def b(self, b):
    self.__b = b

  @staticmethod
  def decode(data):
    f_b = data["b"]

    f_b = A_B.decode(f_b)

    return A(f_b)

  def encode(self):
    data = dict()

    if self.b is None:
      raise Exception("missing required field: b")

    data["b"] = self.b.encode()

    return data

  def __repr__(self):
    return "<A b:{!r}>".format(self.b)

class A_B:
  def __init__(self, field):
    self.__field = field

  @property
  def field(self):
    return self.__field

  @field.setter
  def field(self, field):
    self.__field = field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    if not isinstance(f_field, str):
      raise Exception("not a string")

    return A_B(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("missing required field: field")

    data["field"] = self.field

    return data

  def __repr__(self):
    return "<A_B field:{!r}>".format(self.field)
