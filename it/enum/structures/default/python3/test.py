import enum

class Entry:
  def __init__(self, explicit, implicit):
    self.explicit = explicit
    self.implicit = implicit

  def get_explicit(self):
    return self.explicit

  def get_implicit(self):
    return self.implicit

  @staticmethod
  def decode(data):
    f_explicit = None

    if "explicit" in data:
      f_explicit = data["explicit"]

      if f_explicit is not None:
        f_explicit = EnumExplicit.decode(f_explicit)

    f_implicit = None

    if "implicit" in data:
      f_implicit = data["implicit"]

      if f_implicit is not None:
        f_implicit = EnumImplicit.decode(f_implicit)

    return Entry(f_explicit, f_implicit)

  def encode(self):
    data = dict()

    if self.explicit is not None:
      data["explicit"] = self.explicit.encode()

    if self.implicit is not None:
      data["implicit"] = self.implicit.encode()

    return data

  def __repr__(self):
    return "<Entry explicit:{!r}, implicit:{!r}>".format(self.explicit, self.implicit)

class EnumExplicit:
  def __init__(self, ordinal):
    self.ordinal = ordinal

  def get_ordinal(self):
    return self.ordinal

  def encode(self):
    return self.ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value.ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumExplicit ordinal:{!r}>".format(self.ordinal)

class EnumImplicit:
  def __init__(self, ordinal):
    self.ordinal = ordinal

  def get_ordinal(self):
    return self.ordinal

  def encode(self):
    return self.ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value.ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumImplicit ordinal:{!r}>".format(self.ordinal)

class EnumLongNames:
  def __init__(self, ordinal):
    self.ordinal = ordinal

  def get_ordinal(self):
    return self.ordinal

  def encode(self):
    return self.ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value.ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumLongNames ordinal:{!r}>".format(self.ordinal)

EnumExplicit = enum.Enum("EnumExplicit", [("A", "foo"), ("B", "bar")], type=EnumExplicit)

EnumImplicit = enum.Enum("EnumImplicit", [("A", "A"), ("B", "B")], type=EnumImplicit)

EnumLongNames = enum.Enum("EnumLongNames", [("FooBar", "FooBar"), ("Baz", "Baz")], type=EnumLongNames)
