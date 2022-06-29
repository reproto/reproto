from lower_camel import Value as lower_camel
from lower_snake import Value as lower_snake
from upper_camel import Value as upper_camel
from upper_snake import Value as upper_snake

class Entry:
  def __init__(self, lower_camel, lower_snake, upper_camel, upper_snake):
    self.__lower_camel = lower_camel
    self.__lower_snake = lower_snake
    self.__upper_camel = upper_camel
    self.__upper_snake = upper_snake

  @property
  def lower_camel(self):
    return self.__lower_camel

  @lower_camel.setter
  def lower_camel(self, lower_camel):
    self.__lower_camel = lower_camel

  @property
  def lower_snake(self):
    return self.__lower_snake

  @lower_snake.setter
  def lower_snake(self, lower_snake):
    self.__lower_snake = lower_snake

  @property
  def upper_camel(self):
    return self.__upper_camel

  @upper_camel.setter
  def upper_camel(self, upper_camel):
    self.__upper_camel = upper_camel

  @property
  def upper_snake(self):
    return self.__upper_snake

  @upper_snake.setter
  def upper_snake(self, upper_snake):
    self.__upper_snake = upper_snake

  @staticmethod
  def decode(data):
    f_lower_camel = None

    if "lower_camel" in data:
      f_lower_camel = data["lower_camel"]

      if f_lower_camel is not None:
        f_lower_camel = lower_camel.decode(f_lower_camel)

    f_lower_snake = None

    if "lower_snake" in data:
      f_lower_snake = data["lower_snake"]

      if f_lower_snake is not None:
        f_lower_snake = lower_snake.decode(f_lower_snake)

    f_upper_camel = None

    if "upper_camel" in data:
      f_upper_camel = data["upper_camel"]

      if f_upper_camel is not None:
        f_upper_camel = upper_camel.decode(f_upper_camel)

    f_upper_snake = None

    if "upper_snake" in data:
      f_upper_snake = data["upper_snake"]

      if f_upper_snake is not None:
        f_upper_snake = upper_snake.decode(f_upper_snake)

    return Entry(f_lower_camel, f_lower_snake, f_upper_camel, f_upper_snake)

  def encode(self):
    data = dict()

    if self.lower_camel is not None:
      data["lower_camel"] = self.lower_camel.encode()

    if self.lower_snake is not None:
      data["lower_snake"] = self.lower_snake.encode()

    if self.upper_camel is not None:
      data["upper_camel"] = self.upper_camel.encode()

    if self.upper_snake is not None:
      data["upper_snake"] = self.upper_snake.encode()

    return data

  def __repr__(self):
    return "<Entry lower_camel:{!r}, lower_snake:{!r}, upper_camel:{!r}, upper_snake:{!r}>".format(self.lower_camel, self.lower_snake, self.upper_camel, self.upper_snake)
