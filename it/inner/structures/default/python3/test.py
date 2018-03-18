class Entry:
  def __init__(self, a, b):
    self.a = a
    self.b = b

  def get_a(self):
    return self.a

  def get_b(self):
    return self.b

  @staticmethod
  def decode(data):
    if "a" in data:
      f_a = data["a"]

      if f_a is not None:
        f_a = A.decode(f_a)
    else:
      f_a = None

    if "b" in data:
      f_b = data["b"]

      if f_b is not None:
        f_b = A_B.decode(f_b)
    else:
      f_b = None

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
    self.b = b

  def get_b(self):
    return self.b

  @staticmethod
  def decode(data):
    f_b = A_B.decode(data["b"])

    return A(f_b)

  def encode(self):
    data = dict()

    if self.b is None:
      raise Exception("b: is a required field")

    data["b"] = self.b.encode()

    return data

  def __repr__(self):
    return "<A b:{!r}>".format(self.b)

class A_B:
  def __init__(self, field):
    self.field = field

  def get_field(self):
    return self.field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    return A_B(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field

    return data

  def __repr__(self):
    return "<A_B field:{!r}>".format(self.field)
