class Entry:
  @staticmethod
  def decode(data):
    f_tag = data["@type"]

    if f_tag == "foo":
      return Entry_A.decode(data)

    if f_tag == "b":
      return Entry_B.decode(data)

    if f_tag == "Bar":
      return Entry_Bar.decode(data)

    if f_tag == "Baz":
      return Entry_Baz.decode(data)

    raise Exception("bad type: " + f_tag)

class Entry_A:
  TYPE = "foo"

  def __init__(self, shared):
    self.shared = shared

  def get_shared(self):
    return self.shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    return Entry_A(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "foo"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Entry_A shared:{!r}>".format(self.shared)

class Entry_B:
  TYPE = "b"

  def __init__(self, shared):
    self.shared = shared

  def get_shared(self):
    return self.shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    return Entry_B(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "b"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Entry_B shared:{!r}>".format(self.shared)

class Entry_Bar:
  TYPE = "Bar"

  def __init__(self, shared):
    self.shared = shared

  def get_shared(self):
    return self.shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    return Entry_Bar(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Bar"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Entry_Bar shared:{!r}>".format(self.shared)

class Entry_Baz:
  TYPE = "Baz"

  def __init__(self, shared):
    self.shared = shared

  def get_shared(self):
    return self.shared

  @staticmethod
  def decode(data):
    f_shared = data["shared"]

    return Entry_Baz(f_shared)

  def encode(self):
    data = dict()

    data["@type"] = "Baz"

    if self.shared is None:
      raise Exception("shared: is a required field")

    data["shared"] = self.shared

    return data

  def __repr__(self):
    return "<Entry_Baz shared:{!r}>".format(self.shared)
