class Entry:
  @staticmethod
  def decode(data):
    f_type = data["type"]

    if f_type == "foo":
      return Entry_A.decode(data)

    if f_type == "b":
      return Entry_B.decode(data)

    if f_type == "B":
      return Entry_B.decode(data)

    if f_type == "Bar":
      return Entry_Bar.decode(data)

    if f_type == "Baz":
      return Entry_Baz.decode(data)

    raise Exception("bad type" + f_type)

class Entry_A:
  TYPE = "foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_A()

  def encode(self):
    data = dict()

    data["type"] = "foo"

    return data

  def __repr__(self):
    return "<Entry_A >".format()

class Entry_B:
  TYPE = "b"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_B()

  def encode(self):
    data = dict()

    data["type"] = "b"

    return data

  def __repr__(self):
    return "<Entry_B >".format()

class Entry_Bar:
  TYPE = "Bar"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_Bar()

  def encode(self):
    data = dict()

    data["type"] = "Bar"

    return data

  def __repr__(self):
    return "<Entry_Bar >".format()

class Entry_Baz:
  TYPE = "Baz"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_Baz()

  def encode(self):
    data = dict()

    data["type"] = "Baz"

    return data

  def __repr__(self):
    return "<Entry_Baz >".format()
