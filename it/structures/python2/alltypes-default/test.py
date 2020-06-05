class Entry:
  def __init__(self, _boolean_type, _string_type, _datetime_type, _unsigned_32, _unsigned_64, _signed_32, _signed_64, _float_type, _double_type, _bytes_type, _any_type, _array_type, _array_of_array_type, _map_type):
    self._boolean_type = _boolean_type
    self._string_type = _string_type
    self._datetime_type = _datetime_type
    self._unsigned_32 = _unsigned_32
    self._unsigned_64 = _unsigned_64
    self._signed_32 = _signed_32
    self._signed_64 = _signed_64
    self._float_type = _float_type
    self._double_type = _double_type
    self._bytes_type = _bytes_type
    self._any_type = _any_type
    self._array_type = _array_type
    self._array_of_array_type = _array_of_array_type
    self._map_type = _map_type

  @property
  def boolean_type(self):
    return self._boolean_type

  @property
  def string_type(self):
    return self._string_type

  @property
  def datetime_type(self):
    return self._datetime_type

  @property
  def unsigned_32(self):
    return self._unsigned_32

  @property
  def unsigned_64(self):
    return self._unsigned_64

  @property
  def signed_32(self):
    return self._signed_32

  @property
  def signed_64(self):
    return self._signed_64

  @property
  def float_type(self):
    return self._float_type

  @property
  def double_type(self):
    return self._double_type

  @property
  def bytes_type(self):
    return self._bytes_type

  @property
  def any_type(self):
    return self._any_type

  @property
  def array_type(self):
    return self._array_type

  @property
  def array_of_array_type(self):
    return self._array_of_array_type

  @property
  def map_type(self):
    return self._map_type

  @staticmethod
  def decode(data):
    f_boolean_type = None

    if "boolean_type" in data:
      f_boolean_type = data["boolean_type"]

      if f_boolean_type is not None:
        if not isinstance(f_boolean_type, bool):
          raise Exception("not a boolean")

    f_string_type = None

    if "string_type" in data:
      f_string_type = data["string_type"]

      if f_string_type is not None:
        if not isinstance(f_string_type, unicode):
          raise Exception("not a string")

    f_datetime_type = None

    if "datetime_type" in data:
      f_datetime_type = data["datetime_type"]

      if f_datetime_type is not None:
        if not isinstance(f_datetime_type, unicode):
          raise Exception("not a string")

    f_unsigned_32 = None

    if "unsigned_32" in data:
      f_unsigned_32 = data["unsigned_32"]

      if f_unsigned_32 is not None:
        if not isinstance(f_unsigned_32, int):
          raise Exception("not an integer")

    f_unsigned_64 = None

    if "unsigned_64" in data:
      f_unsigned_64 = data["unsigned_64"]

      if f_unsigned_64 is not None:
        if not isinstance(f_unsigned_64, int):
          raise Exception("not an integer")

    f_signed_32 = None

    if "signed_32" in data:
      f_signed_32 = data["signed_32"]

      if f_signed_32 is not None:
        if not isinstance(f_signed_32, int):
          raise Exception("not an integer")

    f_signed_64 = None

    if "signed_64" in data:
      f_signed_64 = data["signed_64"]

      if f_signed_64 is not None:
        if not isinstance(f_signed_64, int):
          raise Exception("not an integer")

    f_float_type = None

    if "float_type" in data:
      f_float_type = data["float_type"]

      if f_float_type is not None:
        if not isinstance(f_float_type, float):
          raise Exception("not a float")

    f_double_type = None

    if "double_type" in data:
      f_double_type = data["double_type"]

      if f_double_type is not None:
        if not isinstance(f_double_type, float):
          raise Exception("not a float")

    f_bytes_type = None

    if "bytes_type" in data:
      f_bytes_type = data["bytes_type"]

      if f_bytes_type is not None:
        if not isinstance(f_bytes_type, unicode):
          raise Exception("not a string")

    f_any_type = None

    if "any_type" in data:
      f_any_type = data["any_type"]

    f_array_type = None

    if "array_type" in data:
      f_array_type = data["array_type"]

      if f_array_type is not None:
        if not isinstance(f_array_type, list):
          raise Exception("not an array")

        _a0 = []

        for _v0 in f_array_type:
          _v0 = Entry.decode(_v0)
          _a0.append(_v0)

        f_array_type = _a0

    f_array_of_array_type = None

    if "array_of_array_type" in data:
      f_array_of_array_type = data["array_of_array_type"]

      if f_array_of_array_type is not None:
        if not isinstance(f_array_of_array_type, list):
          raise Exception("not an array")

        _a0 = []

        for _v0 in f_array_of_array_type:
          if not isinstance(_v0, list):
            raise Exception("not an array")

          _a1 = []

          for _v1 in _v0:
            _v1 = Entry.decode(_v1)
            _a1.append(_v1)

          _v0 = _a1
          _a0.append(_v0)

        f_array_of_array_type = _a0

    f_map_type = None

    if "map_type" in data:
      f_map_type = data["map_type"]

      if f_map_type is not None:
        if not isinstance(f_map_type, dict):
          raise Exception("not an object")

        _o0 = {}

        for _k0, _v0 in f_map_type.items():
          if not isinstance(_k0, unicode):
            raise Exception("not a string")
          _v0 = Entry.decode(_v0)
          _o0[_k0] = _v0

        f_map_type = _o0

    return Entry(f_boolean_type, f_string_type, f_datetime_type, f_unsigned_32, f_unsigned_64, f_signed_32, f_signed_64, f_float_type, f_double_type, f_bytes_type, f_any_type, f_array_type, f_array_of_array_type, f_map_type)

  def encode(self):
    data = dict()

    if self._boolean_type is not None:
      data["boolean_type"] = self._boolean_type

    if self._string_type is not None:
      data["string_type"] = self._string_type

    if self._datetime_type is not None:
      data["datetime_type"] = self._datetime_type

    if self._unsigned_32 is not None:
      data["unsigned_32"] = self._unsigned_32

    if self._unsigned_64 is not None:
      data["unsigned_64"] = self._unsigned_64

    if self._signed_32 is not None:
      data["signed_32"] = self._signed_32

    if self._signed_64 is not None:
      data["signed_64"] = self._signed_64

    if self._float_type is not None:
      data["float_type"] = self._float_type

    if self._double_type is not None:
      data["double_type"] = self._double_type

    if self._bytes_type is not None:
      data["bytes_type"] = self._bytes_type

    if self._any_type is not None:
      data["any_type"] = self._any_type

    if self._array_type is not None:
      data["array_type"] = [v.encode() for v in self._array_type]

    if self._array_of_array_type is not None:
      data["array_of_array_type"] = [[v.encode() for v in v] for v in self._array_of_array_type]

    if self._map_type is not None:
      data["map_type"] = dict((k, v.encode()) for (k, v) in self._map_type.items())

    return data

  def __repr__(self):
    return "<Entry boolean_type:{!r}, string_type:{!r}, datetime_type:{!r}, unsigned_32:{!r}, unsigned_64:{!r}, signed_32:{!r}, signed_64:{!r}, float_type:{!r}, double_type:{!r}, bytes_type:{!r}, any_type:{!r}, array_type:{!r}, array_of_array_type:{!r}, map_type:{!r}>".format(self._boolean_type, self._string_type, self._datetime_type, self._unsigned_32, self._unsigned_64, self._signed_32, self._signed_64, self._float_type, self._double_type, self._bytes_type, self._any_type, self._array_type, self._array_of_array_type, self._map_type)
