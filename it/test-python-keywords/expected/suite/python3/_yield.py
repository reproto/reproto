class Empty:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Empty()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<Empty >".format()
