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
