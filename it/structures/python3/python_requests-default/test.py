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
    return "<Entry>"

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

  def unknown(self, id):
    """
    UNKNOWN
    """
    path = list()

    path.append(self.url)
    path.append("/")
    path.append("unknown")
    path.append("/")
    path.append(str(id))

    url = "/".join(path)

    r = self.session.request(GET, url=url)

    r.raise_for_status()

  def unknown_return(self, id):
    """
    UNKNOWN
    """
    path = list()

    path.append(self.url)
    path.append("/")
    path.append("unknown-return")
    path.append("/")
    path.append(str(id))

    url = "/".join(path)

    r = self.session.request(GET, url=url)

    r.raise_for_status()

    data = r.json();

    data = Entry.decode(data)

    return data

  def unknown_argument(self, requestid):
    """
    UNKNOWN
    """
    path = list()

    path.append(self.url)
    path.append("/")
    path.append("unknown-argument")
    path.append("/")
    path.append(str(id))

    url = "/".join(path)

    r = self.session.request(GET, url=url)

    r.raise_for_status()

  def unary(self, requestid):
    """
    UNARY
    """
    path = list()

    path.append(self.url)
    path.append("/")
    path.append("unary")
    path.append("/")
    path.append(str(id))

    url = "/".join(path)

    r = self.session.request(GET, url=url)

    r.raise_for_status()

    data = r.json();

    data = Entry.decode(data)

    return data
