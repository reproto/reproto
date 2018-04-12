class Other:
  def __init__(self, name):
    self.name = name

  def get_name(self):
    return self.name

  @staticmethod
  def decode(data):
    f_name = data["name"]

    if not isinstance(f_name, str):
      raise Exception("not a string")

    return Other(f_name)

  def encode(self):
    data = dict()

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    return data

  def __repr__(self):
    return "<Other name:{!r}>".format(self.name)
