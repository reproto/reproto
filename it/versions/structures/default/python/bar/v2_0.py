class Other:
  def __init__(self, _name2):
    self._name2 = _name2

  @property
  def name2(self):
    return self._name2

  @staticmethod
  def decode(data):
    f_name2 = data["name2"]

    if not isinstance(f_name2, unicode):
      raise Exception("not a string")

    return Other(f_name2)

  def encode(self):
    data = dict()

    if self._name2 is None:
      raise Exception("name2: is a required field")

    data["name2"] = self._name2

    return data

  def __repr__(self):
    return "<Other name2:{!r}>".format(self._name2)
