import bar.v1 as bar
import bar.v2_0 as bar2
import bar.v2_1 as bar21

class Thing:
  def __init__(self, _name, _other, _other2, _other21):
    self._name = _name
    self._other = _other
    self._other2 = _other2
    self._other21 = _other21

  @property
  def name(self):
    return self._name

  @property
  def other(self):
    return self._other

  @property
  def other2(self):
    return self._other2

  @property
  def other21(self):
    return self._other21

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

    if self._name is not None:
      data["name"] = self._name

    if self._other is not None:
      data["other"] = self._other.encode()

    if self._other2 is not None:
      data["other2"] = self._other2.encode()

    if self._other21 is not None:
      data["other21"] = self._other21.encode()

    return data

  def __repr__(self):
    return "<Thing name:{!r}, other:{!r}, other2:{!r}, other21:{!r}>".format(self._name, self._other, self._other2, self._other21)
