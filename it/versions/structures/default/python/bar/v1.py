class Other:
  def __init__(self, _name):
    self._name = _name

  @property
  def name(self):
    return self._name

  @staticmethod
  def decode(data):
    f_name = data["name"]

    if not isinstance(f_name, unicode):
      raise Exception("not a string")

    return Other(f_name)

  def encode(self):
    data = dict()

    if self._name is None:
      raise Exception("name: is a required field")

    data["name"] = self._name

    return data

  def __repr__(self):
    return "<Other name:{!r}>".format(self._name)
