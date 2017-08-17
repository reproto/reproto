class Entry:
  def __init__(self, foo):
    self.foo = foo

  @staticmethod
  def decode(data):
    if "foo" in data:
      f_foo = data["foo"]

      if f_foo is not None:
        f_foo = Foo.decode(f_foo)
    else:
      f_foo = None

    return Entry(f_foo)

  def encode(self):
    data = dict()

    if self.foo is not None:
      data["foo"] = self.foo.encode()

    return data

  def __repr__(self):
    return "<Entry foo: {!r}>".format(self.foo)

class Foo:
  def __init__(self, field):
    self.field = field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    return Foo(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field

    return data

  def __repr__(self):
    return "<Foo field: {!r}>".format(self.field)

class Bar:
  def __init__(self, field):
    self.field = field

  @staticmethod
  def decode(data):
    f_field = Bar_Inner.decode(data["field"])

    return Bar(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field.encode()

    return data

  def __repr__(self):
    return "<Bar field: {!r}>".format(self.field)

class Bar_Inner:
  def __init__(self, field):
    self.field = field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    return Bar_Inner(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field

    return data

  def __repr__(self):
    return "<Bar_Inner field: {!r}>".format(self.field)
