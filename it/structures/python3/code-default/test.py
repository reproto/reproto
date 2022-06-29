import enum

class Entry:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<Entry>"

class Type:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Type()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<Type>"

  def type_method(self):
    pass

class Interface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "SubType":
      return Interface_SubType.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

  def interface_method(self):
    pass

class Interface_SubType(Interface):
  TYPE = "SubType"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Interface_SubType()

  def encode(self):
    data = dict()

    data["type"] = "SubType"

    return data

  def __repr__(self):
    return "<Interface_SubType>"

  def subtype_method(self):
    pass

class Enum:
  def __init__(self, _ordinal):
    self.__ordinal = _ordinal

  @property
  def _ordinal(self):
    return self.__ordinal

  @_ordinal.setter
  def _ordinal(self, _ordinal):
    self.__ordinal = _ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<Enum ordinal:{!r}>".format(self._ordinal)

  def enum_method(self):
    pass

class Tuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Tuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<Tuple>"

  def tuple_method(self):
    pass

Enum = enum.Enum("Enum", [("Variant", "Variant")], type=Enum)
