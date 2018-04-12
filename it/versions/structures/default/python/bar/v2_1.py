class Other:
  def __init__(self, name21):
    self.name21 = name21

  def get_name21(self):
    return self.name21

  @staticmethod
  def decode(data):
    f_name21 = data["name21"]

    if not isinstance(f_name21, unicode):
      raise Exception("not a string")

    return Other(f_name21)

  def encode(self):
    data = dict()

    if self.name21 is None:
      raise Exception("name21: is a required field")

    data["name21"] = self.name21

    return data

  def __repr__(self):
    return "<Other name21:{!r}>".format(self.name21)
