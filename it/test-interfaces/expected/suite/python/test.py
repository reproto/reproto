class Entry:
  @staticmethod
  def decode(data):
    f_type = data["type"]

    if f_type == "bar":
      return Entry_Bar.decode(data)

    if f_type == "foo":
      return Entry_Foo.decode(data)

    raise Exception("bad type" + f_type)

class Entry_Bar:
  TYPE = "bar"

  def __init__(self, shared, bar):
    self.shared = shared
    self.bar = bar

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    f_bar = data["bar"]

    return Entry_Bar(f_shared, f_bar)

  def encode(self):
    data = dict()

    data["type"] = "bar"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    if self.bar is None:
      raise Exception("bar: is a required field")

    data["bar"] = self.bar

    return data

  def __repr__(self):
    return "<Entry_Bar shared: {!r}, bar: {!r}>".format(self.shared, self.bar)

class Entry_Foo:
  TYPE = "foo"

  def __init__(self, shared, foo):
    self.shared = shared
    self.foo = foo

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    f_foo = data["foo"]

    return Entry_Foo(f_shared, f_foo)

  def encode(self):
    data = dict()

    data["type"] = "foo"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    if self.foo is None:
      raise Exception("foo: is a required field")

    data["foo"] = self.foo

    return data

  def __repr__(self):
    return "<Entry_Foo shared: {!r}, foo: {!r}>".format(self.shared, self.foo)
