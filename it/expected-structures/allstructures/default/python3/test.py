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
    return "<Entry>".format()

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
    return "<RootType>".format()

class RootInterface:
  @staticmethod
  def decode(data):
    f_tag = data["type"]

    if f_tag == "Foo":
      return RootInterface_Foo.decode(data)

    raise Exception("bad type" + f_tag)

class RootInterface_Foo:
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
    return "<RootInterface_Foo>".format()

class RootEnum:
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
    return "<RootEnum ordinal:{!r}>".format(self.ordinal)

class RootTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple()

  def encode(self):
    return ()

  def __repr__(self):
    return "<RootTuple>".format()


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
    return "<RootType_NestedType>".format()

class RootType_NestedInterface:
  @staticmethod
  def decode(data):
    f_tag = data["type"]

    if f_tag == "Foo":
      return RootType_NestedInterface_Foo.decode(data)

    raise Exception("bad type" + f_tag)

class RootType_NestedInterface_Foo:
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
    return "<RootType_NestedInterface_Foo>".format()

class RootType_NestedEnum:
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
    return "<RootType_NestedEnum ordinal:{!r}>".format(self.ordinal)

class RootType_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootType_NestedTuple()

  def encode(self):
    return ()

  def __repr__(self):
    return "<RootType_NestedTuple>".format()


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
    return "<RootInterface_Foo_NestedType>".format()

class RootInterface_Foo_NestedInterface:
  @staticmethod
  def decode(data):
    f_tag = data["type"]

    if f_tag == "NestedFoo":
      return RootInterface_Foo_NestedInterface_NestedFoo.decode(data)

    raise Exception("bad type" + f_tag)

class RootInterface_Foo_NestedInterface_NestedFoo:
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
    return "<RootInterface_Foo_NestedInterface_NestedFoo>".format()

class RootInterface_Foo_NestedEnum:
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
    return "<RootInterface_Foo_NestedEnum ordinal:{!r}>".format(self.ordinal)

class RootInterface_Foo_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootInterface_Foo_NestedTuple()

  def encode(self):
    return ()

  def __repr__(self):
    return "<RootInterface_Foo_NestedTuple>".format()


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
    return "<RootTuple_NestedType>".format()

class RootTuple_NestedInterface:
  @staticmethod
  def decode(data):
    f_tag = data["type"]

    if f_tag == "Foo":
      return RootTuple_NestedInterface_Foo.decode(data)

    raise Exception("bad type" + f_tag)

class RootTuple_NestedInterface_Foo:
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
    return "<RootTuple_NestedInterface_Foo>".format()

class RootTuple_NestedEnum:
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
    return "<RootTuple_NestedEnum ordinal:{!r}>".format(self.ordinal)

class RootTuple_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootTuple_NestedTuple()

  def encode(self):
    return ()

  def __repr__(self):
    return "<RootTuple_NestedTuple>".format()


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
    return "<RootService_NestedType>".format()

class RootService_NestedInterface:
  @staticmethod
  def decode(data):
    f_tag = data["type"]

    if f_tag == "Foo":
      return RootService_NestedInterface_Foo.decode(data)

    raise Exception("bad type" + f_tag)

class RootService_NestedInterface_Foo:
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
    return "<RootService_NestedInterface_Foo>".format()

class RootService_NestedEnum:
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
    return "<RootService_NestedEnum ordinal:{!r}>".format(self.ordinal)

class RootService_NestedTuple:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return RootService_NestedTuple()

  def encode(self):
    return ()

  def __repr__(self):
    return "<RootService_NestedTuple>".format()


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
    return "<RootType_NestedInterface_Foo_Nested>".format()

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
    return "<RootType_NestedTuple_Nested>".format()

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
    return "<RootType_NestedService_Nested>".format()

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
    return "<RootInterface_Foo_NestedInterface_NestedFoo_Nested>".format()

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
    return "<RootInterface_Foo_NestedTuple_Nested>".format()

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
    return "<RootInterface_Foo_NestedService_Nested>".format()

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
    return "<RootTuple_NestedInterface_Foo_Nested>".format()

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
    return "<RootTuple_NestedTuple_Nested>".format()

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
    return "<RootTuple_NestedService_Nested>".format()

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
    return "<RootService_NestedInterface_Foo_Nested>".format()

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
    return "<RootService_NestedTuple_Nested>".format()

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
    return "<RootService_NestedService_Nested>".format()

RootEnum = enum.Enum("RootEnum", [("Foo", "Foo")], type=RootEnum)

RootType_NestedEnum = enum.Enum("RootType_NestedEnum", [("Foo", "Foo")], type=RootType_NestedEnum)

RootInterface_Foo_NestedEnum = enum.Enum("RootInterface_Foo_NestedEnum", [("Foo", "Foo")], type=RootInterface_Foo_NestedEnum)

RootTuple_NestedEnum = enum.Enum("RootTuple_NestedEnum", [("Foo", "Foo")], type=RootTuple_NestedEnum)

RootService_NestedEnum = enum.Enum("RootService_NestedEnum", [("Foo", "Foo")], type=RootService_NestedEnum)
