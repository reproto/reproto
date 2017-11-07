class Entry:
  @staticmethod
  def decode(data):
    f_type = data["type"]

    if f_type == "bar":
      return Entry_Bar.decode(data)

    if f_type == "foo":
      return Entry_Foo.decode(data)

    raise Exception("bad type" + f_type)

class Entry_Bar:
  TYPE = "bar"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_Bar()

  def encode(self):
    data = dict()

    data["type"] = "bar"

    return data

  def __repr__(self):
    return "<Entry_Bar >".format()

class Entry_Foo:
  TYPE = "foo"

  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry_Foo()

  def encode(self):
    data = dict()

    data["type"] = "foo"

    return data

  def __repr__(self):
    return "<Entry_Foo >".format()
