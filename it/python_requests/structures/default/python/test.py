import requests

class Entry:
  def __init__(self):
    pass

  @staticmethod
  def decode(data):
    return Entry()

  def encode(self):
    data = dict()

    return data

  def __repr__(self):
    return "<Entry>".format()

class MyService_Requests:
  def __init__(self, **kw):
    url = kw.pop("url", None)

    if url is None:
      url = "http://example.com"

    session = kw.pop("session", None)

    if session is None:
      session = requests

    self.url = url
    self.session = session

  def unary(self, request, id):
    """
    UNARY
    """
    path = list()
    path.append(self.url)
    path.append("/")
    path.append("foo")
    path.append("/")
    path.append(str(id))

    url = "".join(path)

    r = self.session.request("POST", url)

    r.raise_for_status()

    data = r.json()

    return Entry.decode(data)

  def main(self):
    """
    """
    r = self.session.request("GET", self.url)

    r.raise_for_status()

    return r.text
