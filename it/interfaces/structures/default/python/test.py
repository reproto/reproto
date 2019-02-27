class Entry:
  def __init__(self, _tagged, _untagged):
    self._tagged = _tagged
    self._untagged = _untagged

  @property
  def tagged(self):
    return self._tagged

  @property
  def untagged(self):
    return self._untagged

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

    if self._tagged is not None:
      data["tagged"] = self._tagged.encode()

    if self._untagged is not None:
      data["untagged"] = self._untagged.encode()

    return data

  def __repr__(self):
    return "<Entry tagged:{!r}, untagged:{!r}>".format(self._tagged, self._untagged)

class Tagged:
  @staticmethod
  def decode(data):
    f_tag = data["@type"]

    if f_tag == "foo":
      return Tagged_A.decode(data)

    if f_tag == "b":
      return Tagged_B.decode(data)

    if f_tag == "Bar":
      return Tagged_Bar.decode(data)

    if f_tag == "Baz":
      return Tagged_Baz.decode(data)

    raise Exception("bad type: " + f_tag)

class Tagged_A:
  TYPE = "foo"

  def __init__(self, _shared):
    self._shared = _shared

  @property
  def shared(self):
    return self._shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    return Tagged_A(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "foo"

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    return data

  def __repr__(self):
    return "<Tagged_A shared:{!r}>".format(self._shared)

class Tagged_B:
  TYPE = "b"

  def __init__(self, _shared):
    self._shared = _shared

  @property
  def shared(self):
    return self._shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    return Tagged_B(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "b"

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    return data

  def __repr__(self):
    return "<Tagged_B shared:{!r}>".format(self._shared)

class Tagged_Bar:
  TYPE = "Bar"

  def __init__(self, _shared):
    self._shared = _shared

  @property
  def shared(self):
    return self._shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    return Tagged_Bar(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Bar"

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    return data

  def __repr__(self):
    return "<Tagged_Bar shared:{!r}>".format(self._shared)

class Tagged_Baz:
  TYPE = "Baz"

  def __init__(self, _shared):
    self._shared = _shared

  @property
  def shared(self):
    return self._shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    return Tagged_Baz(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Baz"

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    return data

  def __repr__(self):
    return "<Tagged_Baz shared:{!r}>".format(self._shared)

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

class Untagged_A:
  TYPE = "A"

  def __init__(self, _shared, _shared_ignore, _a, _b, _ignore):
    self._shared = _shared
    self._shared_ignore = _shared_ignore
    self._a = _a
    self._b = _b
    self._ignore = _ignore

  @property
  def shared(self):
    return self._shared

  @property
  def shared_ignore(self):
    return self._shared_ignore

  @property
  def a(self):
    return self._a

  @property
  def b(self):
    return self._b

  @property
  def ignore(self):
    return self._ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, unicode):
          raise Exception("not a string")

    f_a = data["a"]

    if not isinstance(f_a, unicode):
      raise Exception("not a string")

    f_b = data["b"]

    if not isinstance(f_b, unicode):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, unicode):
          raise Exception("not a string")

    return Untagged_A(f_shared, f_shared_ignore, f_a, f_b, f_ignore)

  def encode(self):
    data = dict()

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    if self._shared_ignore is not None:
      data["shared_ignore"] = self._shared_ignore

    if self._a is None:
      raise Exception("a: is a required field")

    data["a"] = self._a

    if self._b is None:
      raise Exception("b: is a required field")

    data["b"] = self._b

    if self._ignore is not None:
      data["ignore"] = self._ignore

    return data

  def __repr__(self):
    return "<Untagged_A shared:{!r}, shared_ignore:{!r}, a:{!r}, b:{!r}, ignore:{!r}>".format(self._shared, self._shared_ignore, self._a, self._b, self._ignore)

class Untagged_B:
  TYPE = "B"

  def __init__(self, _shared, _shared_ignore, _a, _ignore):
    self._shared = _shared
    self._shared_ignore = _shared_ignore
    self._a = _a
    self._ignore = _ignore

  @property
  def shared(self):
    return self._shared

  @property
  def shared_ignore(self):
    return self._shared_ignore

  @property
  def a(self):
    return self._a

  @property
  def ignore(self):
    return self._ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, unicode):
          raise Exception("not a string")

    f_a = data["a"]

    if not isinstance(f_a, unicode):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, unicode):
          raise Exception("not a string")

    return Untagged_B(f_shared, f_shared_ignore, f_a, f_ignore)

  def encode(self):
    data = dict()

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    if self._shared_ignore is not None:
      data["shared_ignore"] = self._shared_ignore

    if self._a is None:
      raise Exception("a: is a required field")

    data["a"] = self._a

    if self._ignore is not None:
      data["ignore"] = self._ignore

    return data

  def __repr__(self):
    return "<Untagged_B shared:{!r}, shared_ignore:{!r}, a:{!r}, ignore:{!r}>".format(self._shared, self._shared_ignore, self._a, self._ignore)

class Untagged_C:
  TYPE = "C"

  def __init__(self, _shared, _shared_ignore, _b, _ignore):
    self._shared = _shared
    self._shared_ignore = _shared_ignore
    self._b = _b
    self._ignore = _ignore

  @property
  def shared(self):
    return self._shared

  @property
  def shared_ignore(self):
    return self._shared_ignore

  @property
  def b(self):
    return self._b

  @property
  def ignore(self):
    return self._ignore

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    if not isinstance(f_shared, unicode):
      raise Exception("not a string")

    f_shared_ignore = None

    if "shared_ignore" in data:
      f_shared_ignore = data["shared_ignore"]

      if f_shared_ignore is not None:
        if not isinstance(f_shared_ignore, unicode):
          raise Exception("not a string")

    f_b = data["b"]

    if not isinstance(f_b, unicode):
      raise Exception("not a string")

    f_ignore = None

    if "ignore" in data:
      f_ignore = data["ignore"]

      if f_ignore is not None:
        if not isinstance(f_ignore, unicode):
          raise Exception("not a string")

    return Untagged_C(f_shared, f_shared_ignore, f_b, f_ignore)

  def encode(self):
    data = dict()

    if self._shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self._shared

    if self._shared_ignore is not None:
      data["shared_ignore"] = self._shared_ignore

    if self._b is None:
      raise Exception("b: is a required field")

    data["b"] = self._b

    if self._ignore is not None:
      data["ignore"] = self._ignore

    return data

  def __repr__(self):
    return "<Untagged_C shared:{!r}, shared_ignore:{!r}, b:{!r}, ignore:{!r}>".format(self._shared, self._shared_ignore, self._b, self._ignore)
