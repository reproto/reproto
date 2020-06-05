class Value:
  def __init__(self, _foo_bar):
    self._foo_bar = _foo_bar

  @property
  def foo_bar(self):
    return self._foo_bar

  @staticmethod
  def decode(data):
    f_foo_bar = data["fooBar"]

    if not isinstance(f_foo_bar, unicode):
      raise Exception("not a string")

    return Value(f_foo_bar)

  def encode(self):
    data = dict()

    if self._foo_bar is None:
      raise Exception("fooBar: is a required field")

    data["fooBar"] = self._foo_bar

    return data

  def __repr__(self):
    return "<Value foo_bar:{!r}>".format(self._foo_bar)

