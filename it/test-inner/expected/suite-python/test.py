class Entry:
  def __init__(self, a, b):
    self.a = a
    self.b = b

  @staticmethod
  def decode(data):
    f_a = Entry_A.decode(data["a"])

    f_b = Entry_A_B.decode(data["b"])

    return Entry(f_a, f_b)

  def encode(self):
    data = dict()

    if self.a is None:
      raise Exception("a: is a required field")

    data["a"] = self.a.encode()

    if self.b is None:
      raise Exception("b: is a required field")

    data["b"] = self.b.encode()

    return data

  def __repr__(self):
    return "<Entry a: {!r}, b: {!r}>".format(self.a, self.b)

class Entry_A:
  def __init__(self, b):
    self.b = b

  @staticmethod
  def decode(data):
    f_b = Entry_A_B.decode(data["b"])

    return Entry_A(f_b)

  def encode(self):
    data = dict()

    if self.b is None:
      raise Exception("b: is a required field")

    data["b"] = self.b.encode()

    return data

  def __repr__(self):
    return "<Entry_A b: {!r}>".format(self.b)

class Entry_A_B:
  def __init__(self, field):
    self.field = field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    return Entry_A_B(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field

    return data

  def __repr__(self):
    return "<Entry_A_B field: {!r}>".format(self.field)
