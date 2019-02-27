class Entry:
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

    if self._a is not None:
      data["a"] = self._a.encode()

    if self._b is not None:
      data["b"] = self._b.encode()

    return data

  def __repr__(self):
    return "<Entry a:{!r}, b:{!r}>".format(self._a, self._b)

class A:
  def __init__(self, _b):
    self._b = _b

  @property
  def b(self):
    return self._b

  @staticmethod
  def decode(data):
    f_b = data["b"]

    f_b = A_B.decode(f_b)

    return A(f_b)

  def encode(self):
    data = dict()

    if self._b is None:
      raise Exception("b: is a required field")

    data["b"] = self._b.encode()

    return data

  def __repr__(self):
    return "<A b:{!r}>".format(self._b)

class A_B:
  def __init__(self, _field):
    self._field = _field

  @property
  def field(self):
    return self._field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    if not isinstance(f_field, str):
      raise Exception("not a string")

    return A_B(f_field)

  def encode(self):
    data = dict()

    if self._field is None:
      raise Exception("field: is a required field")

    data["field"] = self._field

    return data

  def __repr__(self):
    return "<A_B field:{!r}>".format(self._field)
