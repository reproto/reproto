class Entry:
  def __init__(self, tagged, untagged):
    self.__tagged = tagged
    self.__untagged = untagged

  @property
  def tagged(self):
    return self.__tagged

  @tagged.setter
  def tagged(self, tagged):
    self.__tagged = tagged

  @property
  def untagged(self):
    return self.__untagged

  @untagged.setter
  def untagged(self, untagged):
    self.__untagged = untagged

  @staticmethod
  def decode(data):
    f_tagged = None

    if "tagged" in data:
      f_tagged = data["tagged"]

      if f_tagged is not None:
        f_tagged = Tagged.decode(f_tagged)

    f_untagged = None

    if "untagged" in data:
      f_untagged = data["untagged"]

      if f_untagged is not None:
        f_untagged = Untagged.decode(f_untagged)

    return Entry(f_tagged, f_untagged)

  def encode(self):
    data = dict()

    if self.tagged is not None:
      data["tagged"] = self.tagged.encode()

    if self.untagged is not None:
      data["untagged"] = self.untagged.encode()

    return data

  def __repr__(self):
    return "<Entry tagged:{!r}, untagged:{!r}>".format(self.tagged, self.untagged)

class Tagged:
  @staticmethod
  def decode(data):
    if "@type" not in data:
      raise Exception("missing tag field @type")

    f_tag = data["@type"]

    if f_tag == "foo":
      return Tagged_A.decode(data)

    if f_tag == "b":
      return Tagged_B.decode(data)

    if f_tag == "Bar":
      return Tagged_Bar.decode(data)

    if f_tag == "Baz":
      return Tagged_Baz.decode(data)

    raise Exception("no sub type matching tag: " + f_tag)

class Tagged_A(Tagged):
  TYPE = "foo"

  def __init__(self, shared):
    self.__shared = shared

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    return Tagged_A(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "foo"

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Tagged_A shared:{!r}>".format(self.shared)

class Tagged_B(Tagged):
  TYPE = "b"

  def __init__(self, shared):
    self.__shared = shared

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    return Tagged_B(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "b"

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Tagged_B shared:{!r}>".format(self.shared)

class Tagged_Bar(Tagged):
  TYPE = "Bar"

  def __init__(self, shared):
    self.__shared = shared

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    return Tagged_Bar(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Bar"

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Tagged_Bar shared:{!r}>".format(self.shared)

class Tagged_Baz(Tagged):
  TYPE = "Baz"

  def __init__(self, shared):
    self.__shared = shared

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    return Tagged_Baz(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Baz"

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Tagged_Baz shared:{!r}>".format(self.shared)

class Untagged:
  @staticmethod
  def decode(data):
    keys = set(data.keys())

    if keys >= set(("a", "b")):
      return Untagged_A.decode(data)

    if keys >= set(("a",)):
      return Untagged_B.decode(data)

    if keys >= set(("b",)):
      return Untagged_C.decode(data)

    raise Exception("no sub type matching the given fields: " + repr(keys))

class Untagged_A(Untagged):
  TYPE = "A"

  def __init__(self, shared, shared_ignore, a, b, ignore):
    self.__shared = shared
    self.__shared_ignore = shared_ignore
    self.__a = a
    self.__b = b
    self.__ignore = ignore

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @property
  def shared_ignore(self):
    return self.__shared_ignore

  @shared_ignore.setter
  def shared_ignore(self, shared_ignore):
    self.__shared_ignore = shared_ignore

  @property
  def a(self):
    return self.__a

  @a.setter
  def a(self, a):
    self.__a = a

  @property
  def b(self):
    return self.__b

  @b.setter
  def b(self, b):
    self.__b = b

  @property
  def ignore(self):
    return self.__ignore

  @ignore.setter
  def ignore(self, ignore):
    self.__ignore = ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, str):
          raise Exception("not a string")

    f_a = data["a"]

    if not isinstance(f_a, str):
      raise Exception("not a string")

    f_b = data["b"]

    if not isinstance(f_b, str):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, str):
          raise Exception("not a string")

    return Untagged_A(f_shared, f_shared_ignore, f_a, f_b, f_ignore)

  def encode(self):
    data = dict()

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    if self.shared_ignore is not None:
      data["shared_ignore"] = self.shared_ignore

    if self.a is None:
      raise Exception("missing required field: a")

    data["a"] = self.a

    if self.b is None:
      raise Exception("missing required field: b")

    data["b"] = self.b

    if self.ignore is not None:
      data["ignore"] = self.ignore

    return data

  def __repr__(self):
    return "<Untagged_A shared:{!r}, shared_ignore:{!r}, a:{!r}, b:{!r}, ignore:{!r}>".format(self.shared, self.shared_ignore, self.a, self.b, self.ignore)

class Untagged_B(Untagged):
  TYPE = "B"

  def __init__(self, shared, shared_ignore, a, ignore):
    self.__shared = shared
    self.__shared_ignore = shared_ignore
    self.__a = a
    self.__ignore = ignore

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @property
  def shared_ignore(self):
    return self.__shared_ignore

  @shared_ignore.setter
  def shared_ignore(self, shared_ignore):
    self.__shared_ignore = shared_ignore

  @property
  def a(self):
    return self.__a

  @a.setter
  def a(self, a):
    self.__a = a

  @property
  def ignore(self):
    return self.__ignore

  @ignore.setter
  def ignore(self, ignore):
    self.__ignore = ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, str):
          raise Exception("not a string")

    f_a = data["a"]

    if not isinstance(f_a, str):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, str):
          raise Exception("not a string")

    return Untagged_B(f_shared, f_shared_ignore, f_a, f_ignore)

  def encode(self):
    data = dict()

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    if self.shared_ignore is not None:
      data["shared_ignore"] = self.shared_ignore

    if self.a is None:
      raise Exception("missing required field: a")

    data["a"] = self.a

    if self.ignore is not None:
      data["ignore"] = self.ignore

    return data

  def __repr__(self):
    return "<Untagged_B shared:{!r}, shared_ignore:{!r}, a:{!r}, ignore:{!r}>".format(self.shared, self.shared_ignore, self.a, self.ignore)

class Untagged_C(Untagged):
  TYPE = "C"

  def __init__(self, shared, shared_ignore, b, ignore):
    self.__shared = shared
    self.__shared_ignore = shared_ignore
    self.__b = b
    self.__ignore = ignore

  @property
  def shared(self):
    return self.__shared

  @shared.setter
  def shared(self, shared):
    self.__shared = shared

  @property
  def shared_ignore(self):
    return self.__shared_ignore

  @shared_ignore.setter
  def shared_ignore(self, shared_ignore):
    self.__shared_ignore = shared_ignore

  @property
  def b(self):
    return self.__b

  @b.setter
  def b(self, b):
    self.__b = b

  @property
  def ignore(self):
    return self.__ignore

  @ignore.setter
  def ignore(self, ignore):
    self.__ignore = ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, str):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, str):
          raise Exception("not a string")

    f_b = data["b"]

    if not isinstance(f_b, str):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, str):
          raise Exception("not a string")

    return Untagged_C(f_shared, f_shared_ignore, f_b, f_ignore)

  def encode(self):
    data = dict()

    if self.shared is None:
      raise Exception("missing required field: shared")

    data["shared"] = self.shared

    if self.shared_ignore is not None:
      data["shared_ignore"] = self.shared_ignore

    if self.b is None:
      raise Exception("missing required field: b")

    data["b"] = self.b

    if self.ignore is not None:
      data["ignore"] = self.ignore

    return data

  def __repr__(self):
    return "<Untagged_C shared:{!r}, shared_ignore:{!r}, b:{!r}, ignore:{!r}>".format(self.shared, self.shared_ignore, self.b, self.ignore)
