import bar._1_0_0 as bar
import bar._2_0_0 as bar2

class Thing:
  def __init__(self, name, other, other2):
    self.name = name
    self.other = other
    self.other2 = other2

  def get_name(self):
    return self.name

  def get_other(self):
    return self.other

  def get_other2(self):
    return self.other2

  @staticmethod
  def decode(data):
    if "name" in data:
      f_name = data["name"]

      if f_name is not None:
        f_name = f_name
    else:
      f_name = None

    if "other" in data:
      f_other = data["other"]

      if f_other is not None:
        f_other = bar.Other.decode(f_other)
    else:
      f_other = None

    if "other2" in data:
      f_other2 = data["other2"]

      if f_other2 is not None:
        f_other2 = bar2.Other.decode(f_other2)
    else:
      f_other2 = None

    return Thing(f_name, f_other, f_other2)

  def encode(self):
    data = dict()

    if self.name is not None:
      data["name"] = self.name

    if self.other is not None:
      data["other"] = self.other.encode()

    if self.other2 is not None:
      data["other2"] = self.other2.encode()

    return data

  def __repr__(self):
    return "<Thing name:{!r}, other:{!r}, other2:{!r}>".format(self.name, self.other, self.other2)
