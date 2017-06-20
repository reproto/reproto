class Other:
  def __init__(self, name):
    self.name = name

  @staticmethod
  def decode(data):
    f_name = data["name"]

    return Other(f_name)

  def encode(self):
    data = dict()

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    return data

  def __repr__(self):
    return "<Other name: {!r}>".format(self.name)
