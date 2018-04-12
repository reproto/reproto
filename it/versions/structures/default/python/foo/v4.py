import bar.v1 as bar
import bar.v2_0 as bar2
import bar.v2_1 as bar21

class Thing:
  def __init__(self, name, other, other2, other21):
    self.name = name
    self.other = other
    self.other2 = other2
    self.other21 = other21

  def get_name(self):
    return self.name

  def get_other(self):
    return self.other

  def get_other2(self):
    return self.other2

  def get_other21(self):
    return self.other21

  @staticmethod
  def decode(data):
    f_name = None

    if "name" in data:
      f_name = data["name"]

      if f_name is not None:
        if not isinstance(f_name, unicode):
          raise Exception("not a string")

    f_other = None

    if "other" in data:
      f_other = data["other"]

      if f_other is not None:
        f_other = bar.Other.decode(f_other)

    f_other2 = None

    if "other2" in data:
      f_other2 = data["other2"]

      if f_other2 is not None:
        f_other2 = bar2.Other.decode(f_other2)

    f_other21 = None

    if "other21" in data:
      f_other21 = data["other21"]

      if f_other21 is not None:
        f_other21 = bar21.Other.decode(f_other21)

    return Thing(f_name, f_other, f_other2, f_other21)

  def encode(self):
    data = dict()

    if self.name is not None:
      data["name"] = self.name

    if self.other is not None:
      data["other"] = self.other.encode()

    if self.other2 is not None:
      data["other2"] = self.other2.encode()

    if self.other21 is not None:
      data["other21"] = self.other21.encode()

    return data

  def __repr__(self):
    return "<Thing name:{!r}, other:{!r}, other2:{!r}, other21:{!r}>".format(self.name, self.other, self.other2, self.other21)
