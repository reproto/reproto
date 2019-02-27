import foo.v4 as foo

class Entry:
  def __init__(self, _thing):
    self._thing = _thing

  @property
  def thing(self):
    return self._thing

  @staticmethod
  def decode(data):
    f_thing = None

    if "thing" in data:
      f_thing = data["thing"]

      if f_thing is not None:
        f_thing = foo.Thing.decode(f_thing)

    return Entry(f_thing)

  def encode(self):
    data = dict()

    if self._thing is not None:
      data["thing"] = self._thing.encode()

    return data

  def __repr__(self):
    return "<Entry thing:{!r}>".format(self._thing)
