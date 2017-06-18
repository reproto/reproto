class Thing:
  def __init__(self, name):
    self.name = name

  @staticmethod
  def decode(data):
    f_name = data["name"]

    return Thing(f_name)

  def encode(self):
    data = dict()

    if self.name is None:
      raise Exception("name: is a required field")

    data["name"] = self.name

    return data

  def __repr__(self):
    return "<Thing name: {!r}>".format(self.name)
