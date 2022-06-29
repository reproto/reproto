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

class RootType:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootType>"

class RootInterface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "Foo":
      return RootInterface_Foo.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class RootInterface_Foo(RootInterface):
  TYPE = "Foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo()

  def encode(self):
    data = dict()

    data["type"] = "Foo"

    return data

  def __repr__(self):
    return "<RootInterface_Foo>"

class RootEnum:
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
    return "<RootEnum ordinal:{!r}>".format(self._ordinal)

class RootTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<RootTuple>"

class RootType_NestedType:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedType()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootType_NestedType>"

class RootType_NestedInterface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "Foo":
      return RootType_NestedInterface_Foo.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class RootType_NestedInterface_Foo(RootType_NestedInterface):
  TYPE = "Foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedInterface_Foo()

  def encode(self):
    data = dict()

    data["type"] = "Foo"

    return data

  def __repr__(self):
    return "<RootType_NestedInterface_Foo>"

class RootType_NestedEnum:
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
    return "<RootType_NestedEnum ordinal:{!r}>".format(self._ordinal)

class RootType_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedTuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<RootType_NestedTuple>"

class RootInterface_Foo_NestedType:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedType()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootInterface_Foo_NestedType>"

class RootInterface_Foo_NestedInterface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "NestedFoo":
      return RootInterface_Foo_NestedInterface_NestedFoo.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class RootInterface_Foo_NestedInterface_NestedFoo(RootInterface_Foo_NestedInterface):
  TYPE = "NestedFoo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedInterface_NestedFoo()

  def encode(self):
    data = dict()

    data["type"] = "NestedFoo"

    return data

  def __repr__(self):
    return "<RootInterface_Foo_NestedInterface_NestedFoo>"

class RootInterface_Foo_NestedEnum:
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
    return "<RootInterface_Foo_NestedEnum ordinal:{!r}>".format(self._ordinal)

class RootInterface_Foo_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedTuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<RootInterface_Foo_NestedTuple>"

class RootTuple_NestedType:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedType()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootTuple_NestedType>"

class RootTuple_NestedInterface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "Foo":
      return RootTuple_NestedInterface_Foo.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class RootTuple_NestedInterface_Foo(RootTuple_NestedInterface):
  TYPE = "Foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedInterface_Foo()

  def encode(self):
    data = dict()

    data["type"] = "Foo"

    return data

  def __repr__(self):
    return "<RootTuple_NestedInterface_Foo>"

class RootTuple_NestedEnum:
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
    return "<RootTuple_NestedEnum ordinal:{!r}>".format(self._ordinal)

class RootTuple_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedTuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<RootTuple_NestedTuple>"

class RootService_NestedType:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedType()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootService_NestedType>"

class RootService_NestedInterface:
  @staticmethod
  def decode(data):
    if "type" not in data:
      raise Exception("missing tag field type")

    f_tag = data["type"]

    if f_tag == "Foo":
      return RootService_NestedInterface_Foo.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class RootService_NestedInterface_Foo(RootService_NestedInterface):
  TYPE = "Foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedInterface_Foo()

  def encode(self):
    data = dict()

    data["type"] = "Foo"

    return data

  def __repr__(self):
    return "<RootService_NestedInterface_Foo>"

class RootService_NestedEnum:
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
    return "<RootService_NestedEnum ordinal:{!r}>".format(self._ordinal)

class RootService_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedTuple()

  def encode(self):

    return ()

  def __repr__(self):
    return "<RootService_NestedTuple>"

class RootType_NestedInterface_Foo_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedInterface_Foo_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootType_NestedInterface_Foo_Nested>"

class RootType_NestedTuple_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedTuple_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootType_NestedTuple_Nested>"

class RootType_NestedService_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedService_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootType_NestedService_Nested>"

class RootInterface_Foo_NestedInterface_NestedFoo_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedInterface_NestedFoo_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootInterface_Foo_NestedInterface_NestedFoo_Nested>"

class RootInterface_Foo_NestedTuple_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedTuple_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootInterface_Foo_NestedTuple_Nested>"

class RootInterface_Foo_NestedService_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedService_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootInterface_Foo_NestedService_Nested>"

class RootTuple_NestedInterface_Foo_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedInterface_Foo_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootTuple_NestedInterface_Foo_Nested>"

class RootTuple_NestedTuple_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedTuple_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootTuple_NestedTuple_Nested>"

class RootTuple_NestedService_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedService_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootTuple_NestedService_Nested>"

class RootService_NestedInterface_Foo_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedInterface_Foo_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootService_NestedInterface_Foo_Nested>"

class RootService_NestedTuple_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedTuple_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootService_NestedTuple_Nested>"

class RootService_NestedService_Nested:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedService_Nested()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<RootService_NestedService_Nested>"

RootEnum = enum.Enum("RootEnum", [("Foo", "Foo")], type=RootEnum)

RootType_NestedEnum = enum.Enum("RootType_NestedEnum", [("Foo", "Foo")], type=RootType_NestedEnum)

RootInterface_Foo_NestedEnum = enum.Enum("RootInterface_Foo_NestedEnum", [("Foo", "Foo")], type=RootInterface_Foo_NestedEnum)

RootTuple_NestedEnum = enum.Enum("RootTuple_NestedEnum", [("Foo", "Foo")], type=RootTuple_NestedEnum)

RootService_NestedEnum = enum.Enum("RootService_NestedEnum", [("Foo", "Foo")], type=RootService_NestedEnum)
