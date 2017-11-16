class Entry:
  def __init__(self, boolean_type, string_type, datetime_type, unsigned_32, unsigned_64, signed_32, signed_64, float_type, double_type, bytes_type, any_type, array_type, map_type):
    self.boolean_type = boolean_type
    self.string_type = string_type
    self.datetime_type = datetime_type
    self.unsigned_32 = unsigned_32
    self.unsigned_64 = unsigned_64
    self.signed_32 = signed_32
    self.signed_64 = signed_64
    self.float_type = float_type
    self.double_type = double_type
    self.bytes_type = bytes_type
    self.any_type = any_type
    self.array_type = array_type
    self.map_type = map_type

  @staticmethod
  def decode(data):
    if "boolean_type" in data:
      f_boolean_type = data["boolean_type"]

      if f_boolean_type is not None:
        f_boolean_type = f_boolean_type
    else:
      f_boolean_type = None

    if "string_type" in data:
      f_string_type = data["string_type"]

      if f_string_type is not None:
        f_string_type = f_string_type
    else:
      f_string_type = None

    if "datetime_type" in data:
      f_datetime_type = data["datetime_type"]

      if f_datetime_type is not None:
        f_datetime_type = f_datetime_type
    else:
      f_datetime_type = None

    if "unsigned_32" in data:
      f_unsigned_32 = data["unsigned_32"]

      if f_unsigned_32 is not None:
        f_unsigned_32 = f_unsigned_32
    else:
      f_unsigned_32 = None

    if "unsigned_64" in data:
      f_unsigned_64 = data["unsigned_64"]

      if f_unsigned_64 is not None:
        f_unsigned_64 = f_unsigned_64
    else:
      f_unsigned_64 = None

    if "signed_32" in data:
      f_signed_32 = data["signed_32"]

      if f_signed_32 is not None:
        f_signed_32 = f_signed_32
    else:
      f_signed_32 = None

    if "signed_64" in data:
      f_signed_64 = data["signed_64"]

      if f_signed_64 is not None:
        f_signed_64 = f_signed_64
    else:
      f_signed_64 = None

    if "float_type" in data:
      f_float_type = data["float_type"]

      if f_float_type is not None:
        f_float_type = f_float_type
    else:
      f_float_type = None

    if "double_type" in data:
      f_double_type = data["double_type"]

      if f_double_type is not None:
        f_double_type = f_double_type
    else:
      f_double_type = None

    if "bytes_type" in data:
      f_bytes_type = data["bytes_type"]

      if f_bytes_type is not None:
        f_bytes_type = f_bytes_type
    else:
      f_bytes_type = None

    if "any_type" in data:
      f_any_type = data["any_type"]

      if f_any_type is not None:
        f_any_type = f_any_type
    else:
      f_any_type = None

    if "array_type" in data:
      f_array_type = data["array_type"]

      if f_array_type is not None:
        f_array_type = [Entry.decode(v) for v in f_array_type]
    else:
      f_array_type = None

    if "map_type" in data:
      f_map_type = data["map_type"]

      if f_map_type is not None:
        f_map_type = dict((k, Entry.decode(v)) for (k, v) in f_map_type.items())
    else:
      f_map_type = None

    return Entry(f_boolean_type, f_string_type, f_datetime_type, f_unsigned_32, f_unsigned_64, f_signed_32, f_signed_64, f_float_type, f_double_type, f_bytes_type, f_any_type, f_array_type, f_map_type)

  def encode(self):
    data = dict()

    if self.boolean_type is not None:
      data["boolean_type"] = self.boolean_type

    if self.string_type is not None:
      data["string_type"] = self.string_type

    if self.datetime_type is not None:
      data["datetime_type"] = self.datetime_type

    if self.unsigned_32 is not None:
      data["unsigned_32"] = self.unsigned_32

    if self.unsigned_64 is not None:
      data["unsigned_64"] = self.unsigned_64

    if self.signed_32 is not None:
      data["signed_32"] = self.signed_32

    if self.signed_64 is not None:
      data["signed_64"] = self.signed_64

    if self.float_type is not None:
      data["float_type"] = self.float_type

    if self.double_type is not None:
      data["double_type"] = self.double_type

    if self.bytes_type is not None:
      data["bytes_type"] = self.bytes_type

    if self.any_type is not None:
      data["any_type"] = self.any_type

    if self.array_type is not None:
      data["array_type"] = [v.encode() for v in self.array_type]

    if self.map_type is not None:
      data["map_type"] = dict((k, v.encode()) for (k, v) in self.map_type.items())

    return data

  def __repr__(self):
    return "<Entry boolean_type: {!r}, string_type: {!r}, datetime_type: {!r}, unsigned_32: {!r}, unsigned_64: {!r}, signed_32: {!r}, signed_64: {!r}, float_type: {!r}, double_type: {!r}, bytes_type: {!r}, any_type: {!r}, array_type: {!r}, map_type: {!r}>".format(self.boolean_type, self.string_type, self.datetime_type, self.unsigned_32, self.unsigned_64, self.signed_32, self.signed_64, self.float_type, self.double_type, self.bytes_type, self.any_type, self.array_type, self.map_type)
