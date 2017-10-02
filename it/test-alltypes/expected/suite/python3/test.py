class Entry:
  def __init__(self, string_type, unsigned_type, unsigned_sized_type, signed_type, signed_sized_type, float_type, double_type, array_type, map_type):
    self.string_type = string_type
    self.unsigned_type = unsigned_type
    self.unsigned_sized_type = unsigned_sized_type
    self.signed_type = signed_type
    self.signed_sized_type = signed_sized_type
    self.float_type = float_type
    self.double_type = double_type
    self.array_type = array_type
    self.map_type = map_type

  @staticmethod
  def decode(data):
    if "string_type" in data:
      f_string_type = data["string_type"]

      if f_string_type is not None:
        f_string_type = f_string_type
    else:
      f_string_type = None

    if "unsigned_type" in data:
      f_unsigned_type = data["unsigned_type"]

      if f_unsigned_type is not None:
        f_unsigned_type = f_unsigned_type
    else:
      f_unsigned_type = None

    if "unsigned_sized_type" in data:
      f_unsigned_sized_type = data["unsigned_sized_type"]

      if f_unsigned_sized_type is not None:
        f_unsigned_sized_type = f_unsigned_sized_type
    else:
      f_unsigned_sized_type = None

    if "signed_type" in data:
      f_signed_type = data["signed_type"]

      if f_signed_type is not None:
        f_signed_type = f_signed_type
    else:
      f_signed_type = None

    if "signed_sized_type" in data:
      f_signed_sized_type = data["signed_sized_type"]

      if f_signed_sized_type is not None:
        f_signed_sized_type = f_signed_sized_type
    else:
      f_signed_sized_type = None

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

    return Entry(f_string_type, f_unsigned_type, f_unsigned_sized_type, f_signed_type, f_signed_sized_type, f_float_type, f_double_type, f_array_type, f_map_type)

  def encode(self):
    data = dict()

    if self.string_type is not None:
      data["string_type"] = self.string_type

    if self.unsigned_type is not None:
      data["unsigned_type"] = self.unsigned_type

    if self.unsigned_sized_type is not None:
      data["unsigned_sized_type"] = self.unsigned_sized_type

    if self.signed_type is not None:
      data["signed_type"] = self.signed_type

    if self.signed_sized_type is not None:
      data["signed_sized_type"] = self.signed_sized_type

    if self.float_type is not None:
      data["float_type"] = self.float_type

    if self.double_type is not None:
      data["double_type"] = self.double_type

    if self.array_type is not None:
      data["array_type"] = [v.encode() for v in self.array_type]

    if self.map_type is not None:
      data["map_type"] = dict((k, v.encode()) for (k, v) in self.map_type.items())

    return data

  def __repr__(self):
    return "<Entry string_type: {!r}, unsigned_type: {!r}, unsigned_sized_type: {!r}, signed_type: {!r}, signed_sized_type: {!r}, float_type: {!r}, double_type: {!r}, array_type: {!r}, map_type: {!r}>".format(self.string_type, self.unsigned_type, self.unsigned_sized_type, self.signed_type, self.signed_sized_type, self.float_type, self.double_type, self.array_type, self.map_type)
