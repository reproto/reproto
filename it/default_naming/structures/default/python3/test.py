import lower_camel as lower_camel
import lower_snake as lower_snake
import upper_camel as upper_camel
import upper_snake as upper_snake

class Entry:
  def __init__(self, _lower_camel, _lower_snake, _upper_camel, _upper_snake):
    self._lower_camel = _lower_camel
    self._lower_snake = _lower_snake
    self._upper_camel = _upper_camel
    self._upper_snake = _upper_snake

  @property
  def lower_camel(self):
    return self._lower_camel

  @property
  def lower_snake(self):
    return self._lower_snake

  @property
  def upper_camel(self):
    return self._upper_camel

  @property
  def upper_snake(self):
    return self._upper_snake

  @staticmethod
  def decode(data):
    f_lower_camel = None

    if "lower_camel" in data:
      f_lower_camel = data["lower_camel"]

      if f_lower_camel is not None:
        f_lower_camel = lower_camel.Value.decode(f_lower_camel)

    f_lower_snake = None

    if "lower_snake" in data:
      f_lower_snake = data["lower_snake"]

      if f_lower_snake is not None:
        f_lower_snake = lower_snake.Value.decode(f_lower_snake)

    f_upper_camel = None

    if "upper_camel" in data:
      f_upper_camel = data["upper_camel"]

      if f_upper_camel is not None:
        f_upper_camel = upper_camel.Value.decode(f_upper_camel)

    f_upper_snake = None

    if "upper_snake" in data:
      f_upper_snake = data["upper_snake"]

      if f_upper_snake is not None:
        f_upper_snake = upper_snake.Value.decode(f_upper_snake)

    return Entry(f_lower_camel, f_lower_snake, f_upper_camel, f_upper_snake)

  def encode(self):
    data = dict()

    if self._lower_camel is not None:
      data["lower_camel"] = self._lower_camel.encode()

    if self._lower_snake is not None:
      data["lower_snake"] = self._lower_snake.encode()

    if self._upper_camel is not None:
      data["upper_camel"] = self._upper_camel.encode()

    if self._upper_snake is not None:
      data["upper_snake"] = self._upper_snake.encode()

    return data

  def __repr__(self):
    return "<Entry lower_camel:{!r}, lower_snake:{!r}, upper_camel:{!r}, upper_snake:{!r}>".format(self._lower_camel, self._lower_snake, self._upper_camel, self._upper_snake)
