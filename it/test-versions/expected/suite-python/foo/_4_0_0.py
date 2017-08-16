import bar._1_0_0 as bar
import bar._2_0_0 as bar2

class Thing:
  def __init__(self, name, other, other2):
    self.name = name
    self.other = other
    self.other2 = other2

  @staticmethod
  def decode(data):
    f_name = data["name"]

    f_other = bar.Other.decode(data["other"])

    f_other2 = bar2.Other.decode(data["other2"])

    return Thing(f_name, f_other, f_other2)

  def encode(self):
    data = dict()

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    if self.other is None:
      raise Exception("other: is a required field")

    data["other"] = self.other.encode()

    if self.other2 is None:
      raise Exception("other2: is a required field")

    data["other2"] = self.other2.encode()

    return data

  def __repr__(self):
    return "<Thing name: {!r}, other: {!r}, other2: {!r}>".format(self.name, self.other, self.other2)
