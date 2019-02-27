class Other:
  def __init__(self, _name21):
    self._name21 = _name21

  @property
  def name21(self):
    return self._name21

  @staticmethod
  def decode(data):
    f_name21 = data["name21"]

    if not isinstance(f_name21, str):
      raise Exception("not a string")

    return Other(f_name21)

  def encode(self):
    data = dict()

    if self._name21 is None:
      raise Exception("name21: is a required field")

    data["name21"] = self._name21

    return data

  def __repr__(self):
    return "<Other name21:{!r}>".format(self._name21)
