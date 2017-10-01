import enum

class Entry:
  def __init__(self, explicit, implicit):
    self.explicit = explicit
    self.implicit = implicit

  @staticmethod
  def decode(data):
    f_explicit = EnumExplicit.decode(data["explicit"])

    f_implicit = EnumImplicit.decode(data["implicit"])

    return Entry(f_explicit, f_implicit)

  def encode(self):
    data = dict()

    if self.explicit is None:
      raise Exception("explicit: is a required field")

    data["explicit"] = self.explicit.encode()

    if self.implicit is None:
      raise Exception("implicit: is a required field")

    data["implicit"] = self.implicit.encode()

    return data

  def __repr__(self):
    return "<Entry explicit: {!r}, implicit: {!r}>".format(self.explicit, self.implicit)

class EnumExplicit:
  def __init__(self, _value):
    self._value = _value

  def encode(self):
    return self._value

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._value == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumExplicit _value: {!r}>".format(self._value)

class EnumImplicit:
  def __init__(self, _value):
    self._value = _value

  def encode(self):
    return self._value

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._value == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumImplicit _value: {!r}>".format(self._value)

EnumExplicit = enum.Enum("EnumExplicit", [("A", "foo"), ("B", "bar")], type=EnumExplicit)

EnumImplicit = enum.Enum("EnumImplicit", [("A", "A"), ("B", "B")], type=EnumImplicit)
