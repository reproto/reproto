class Value:
  def __init__(self, foo_bar):
    self.__foo_bar = foo_bar

  @property
  def foo_bar(self):
    return self.__foo_bar

  @foo_bar.setter
  def foo_bar(self, foo_bar):
    self.__foo_bar = foo_bar

  @staticmethod
  def decode(data):
    f_foo_bar = data["FOO_BAR"]

    if not isinstance(f_foo_bar, unicode):
      raise Exception("not a string")

    return Value(f_foo_bar)

  def encode(self):
    data = dict()

    if self.foo_bar is None:
      raise Exception("missing required field: foo_bar")

    data["FOO_BAR"] = self.foo_bar

    return data

  def __repr__(self):
    return "<Value foo_bar:{!r}>".format(self.foo_bar)
