class Entry:
  def __init__(self, _foo):
    self._foo = _foo

  @property
  def foo(self):
    """
    The foo field.
    """
    return self._foo

  @staticmethod
  def decode(data):
    f_foo = None

    if "foo" in data:
      f_foo = data["foo"]

      if f_foo is not None:
        f_foo = Foo.decode(f_foo)

    return Entry(f_foo)

  def encode(self):
    data = dict()

    if self._foo is not None:
      data["foo"] = self._foo.encode()

    return data

  def __repr__(self):
    return "<Entry foo:{!r}>".format(self._foo)

class Foo:
  def __init__(self, _field):
    self._field = _field

  @property
  def field(self):
    """
    The field.
    """
    return self._field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    if not isinstance(f_field, str):
      raise Exception("not a string")

    return Foo(f_field)

  def encode(self):
    data = dict()

    if self._field is None:
      raise Exception("field: is a required field")

    data["field"] = self._field

    return data

  def __repr__(self):
    return "<Foo field:{!r}>".format(self._field)

class Bar:
  def __init__(self, _field):
    self._field = _field

  @property
  def field(self):
    """
    The inner field.
    """
    return self._field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    f_field = Bar_Inner.decode(f_field)

    return Bar(f_field)

  def encode(self):
    data = dict()

    if self._field is None:
      raise Exception("field: is a required field")

    data["field"] = self._field.encode()

    return data

  def __repr__(self):
    return "<Bar field:{!r}>".format(self._field)

class Bar_Inner:
  def __init__(self, _field):
    self._field = _field

  @property
  def field(self):
    """
    The field.
    """
    return self._field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    if not isinstance(f_field, str):
      raise Exception("not a string")

    return Bar_Inner(f_field)

  def encode(self):
    data = dict()

    if self._field is None:
      raise Exception("field: is a required field")

    data["field"] = self._field

    return data

  def __repr__(self):
    return "<Bar_Inner field:{!r}>".format(self._field)
