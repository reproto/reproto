import enum

class Entry:
  def __init__(self, _explicit, _implicit, _enum_u32, _enum_u64, _enum_i32, _enum_i64):
    self._explicit = _explicit
    self._implicit = _implicit
    self._enum_u32 = _enum_u32
    self._enum_u64 = _enum_u64
    self._enum_i32 = _enum_i32
    self._enum_i64 = _enum_i64

  @property
  def explicit(self):
    return self._explicit

  @property
  def implicit(self):
    return self._implicit

  @property
  def enum_u32(self):
    return self._enum_u32

  @property
  def enum_u64(self):
    return self._enum_u64

  @property
  def enum_i32(self):
    return self._enum_i32

  @property
  def enum_i64(self):
    return self._enum_i64

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

    f_enum_u32 = None

    if "enum_u32" in data:
      f_enum_u32 = data["enum_u32"]

      if f_enum_u32 is not None:
        f_enum_u32 = EnumU32.decode(f_enum_u32)

    f_enum_u64 = None

    if "enum_u64" in data:
      f_enum_u64 = data["enum_u64"]

      if f_enum_u64 is not None:
        f_enum_u64 = EnumU64.decode(f_enum_u64)

    f_enum_i32 = None

    if "enum_i32" in data:
      f_enum_i32 = data["enum_i32"]

      if f_enum_i32 is not None:
        f_enum_i32 = EnumI32.decode(f_enum_i32)

    f_enum_i64 = None

    if "enum_i64" in data:
      f_enum_i64 = data["enum_i64"]

      if f_enum_i64 is not None:
        f_enum_i64 = EnumI64.decode(f_enum_i64)

    return Entry(f_explicit, f_implicit, f_enum_u32, f_enum_u64, f_enum_i32, f_enum_i64)

  def encode(self):
    data = dict()

    if self._explicit is not None:
      data["explicit"] = self._explicit.encode()

    if self._implicit is not None:
      data["implicit"] = self._implicit.encode()

    if self._enum_u32 is not None:
      data["enum_u32"] = self._enum_u32.encode()

    if self._enum_u64 is not None:
      data["enum_u64"] = self._enum_u64.encode()

    if self._enum_i32 is not None:
      data["enum_i32"] = self._enum_i32.encode()

    if self._enum_i64 is not None:
      data["enum_i64"] = self._enum_i64.encode()

    return data

  def __repr__(self):
    return "<Entry explicit:{!r}, implicit:{!r}, enum_u32:{!r}, enum_u64:{!r}, enum_i32:{!r}, enum_i64:{!r}>".format(self._explicit, self._implicit, self._enum_u32, self._enum_u64, self._enum_i32, self._enum_i64)

class EnumExplicit:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumExplicit ordinal:{!r}>".format(self._ordinal)

class EnumImplicit:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumImplicit ordinal:{!r}>".format(self._ordinal)

class EnumLongNames:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumLongNames ordinal:{!r}>".format(self._ordinal)

class EnumU32:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumU32 ordinal:{!r}>".format(self._ordinal)

class EnumU64:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumU64 ordinal:{!r}>".format(self._ordinal)

class EnumI32:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumI32 ordinal:{!r}>".format(self._ordinal)

class EnumI64:
  def __init__(self, _ordinal):
    self._ordinal = _ordinal

  @property
  def ordinal(self):
    return self._ordinal

  def encode(self):
    return self._ordinal

  @classmethod
  def decode(cls, data):
    for value in cls.__members__.values():
      if value._ordinal == data:
        return value

    raise Exception("data does not match enum")

  def __repr__(self):
    return "<EnumI64 ordinal:{!r}>".format(self._ordinal)

EnumExplicit = enum.Enum("EnumExplicit", [("A", "foo"), ("B", "bar")], type=EnumExplicit)

EnumImplicit = enum.Enum("EnumImplicit", [("A", "A"), ("B", "B")], type=EnumImplicit)

EnumLongNames = enum.Enum("EnumLongNames", [("FooBar", "FooBar"), ("Baz", "Baz")], type=EnumLongNames)

EnumU32 = enum.Enum("EnumU32", [("Min", 0), ("Max", 2147483647)], type=EnumU32)

EnumU64 = enum.Enum("EnumU64", [("Min", 0), ("Max", 9007199254740991)], type=EnumU64)

EnumI32 = enum.Enum("EnumI32", [("Min", -2147483648), ("NegativeOne", -1), ("Zero", 0), ("Max", 2147483647)], type=EnumI32)

EnumI64 = enum.Enum("EnumI64", [("Min", -9007199254740991), ("NegativeOne", -1), ("Zero", 0), ("Max", 9007199254740991)], type=EnumI64)
