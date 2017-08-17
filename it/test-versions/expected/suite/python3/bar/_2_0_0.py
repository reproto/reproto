class Other:
  def __init__(self, name2):
    self.name2 = name2

  @staticmethod
  def decode(data):
    f_name2 = data["name2"]

    return Other(f_name2)

  def encode(self):
    data = dict()

    if self.name2 is None:
      raise Exception("name2: is a required field")

    data["name2"] = self.name2

    return data

  def __repr__(self):
    return "<Other name2: {!r}>".format(self.name2)
