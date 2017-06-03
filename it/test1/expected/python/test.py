class Foo:
  def __init__(self, field):
    self.field = field

  @staticmethod
  def decode(data):
    f_field = data["field"]

    return Foo(f_field)

  def encode(self):
    data = dict()

    if self.field is None:
      raise Exception("field: is a required field")

    data["field"] = self.field

    return data
