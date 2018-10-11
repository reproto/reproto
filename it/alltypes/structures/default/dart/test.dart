class Entry {
  bool booleanType;
  String stringType;
  String datetimeType;
  int unsigned32;
  int unsigned64;
  int signed32;
  int signed64;
  double floatType;
  double doubleType;
  String bytesType;
  dynamic anyType;
  List<Entry> arrayType;
  List<List<Entry>> arrayOfArrayType;
  Map<String, Entry> mapType;

  Entry(
    this.booleanType,
    this.stringType,
    this.datetimeType,
    this.unsigned32,
    this.unsigned64,
    this.signed32,
    this.signed64,
    this.floatType,
    this.doubleType,
    this.bytesType,
    this.anyType,
    this.arrayType,
    this.arrayOfArrayType,
    this.mapType
  );

  static Entry decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var booleanType_dyn = _data["boolean_type"];
    bool booleanType = null;
    if (booleanType_dyn != null) {
      if (!(booleanType_dyn is bool)) {
        throw 'expected bool, but was: $booleanType_dyn';
      }
      booleanType = booleanType_dyn;
    }

    var stringType_dyn = _data["string_type"];
    String stringType = null;
    if (stringType_dyn != null) {
      if (!(stringType_dyn is String)) {
        throw 'expected String, but was: $stringType_dyn';
      }
      stringType = stringType_dyn;
    }

    var datetimeType_dyn = _data["datetime_type"];
    String datetimeType = null;
    if (datetimeType_dyn != null) {
      if (!(datetimeType_dyn is String)) {
        throw 'expected String, but was: $datetimeType_dyn';
      }
      datetimeType = datetimeType_dyn;
    }

    var unsigned32_dyn = _data["unsigned_32"];
    int unsigned32 = null;
    if (unsigned32_dyn != null) {
      if (!(unsigned32_dyn is int)) {
        throw 'expected int, but was: $unsigned32_dyn';
      }
      unsigned32 = unsigned32_dyn;
    }

    var unsigned64_dyn = _data["unsigned_64"];
    int unsigned64 = null;
    if (unsigned64_dyn != null) {
      if (!(unsigned64_dyn is int)) {
        throw 'expected int, but was: $unsigned64_dyn';
      }
      unsigned64 = unsigned64_dyn;
    }

    var signed32_dyn = _data["signed_32"];
    int signed32 = null;
    if (signed32_dyn != null) {
      if (!(signed32_dyn is int)) {
        throw 'expected int, but was: $signed32_dyn';
      }
      signed32 = signed32_dyn;
    }

    var signed64_dyn = _data["signed_64"];
    int signed64 = null;
    if (signed64_dyn != null) {
      if (!(signed64_dyn is int)) {
        throw 'expected int, but was: $signed64_dyn';
      }
      signed64 = signed64_dyn;
    }

    var floatType_dyn = _data["float_type"];
    double floatType = null;
    if (floatType_dyn != null) {
      if (!(floatType_dyn is double)) {
        throw 'expected double, but was: $floatType_dyn';
      }
      floatType = floatType_dyn;
    }

    var doubleType_dyn = _data["double_type"];
    double doubleType = null;
    if (doubleType_dyn != null) {
      if (!(doubleType_dyn is double)) {
        throw 'expected double, but was: $doubleType_dyn';
      }
      doubleType = doubleType_dyn;
    }

    var bytesType_dyn = _data["bytes_type"];
    String bytesType = null;
    if (bytesType_dyn != null) {
      if (!(bytesType_dyn is String)) {
        throw 'expected String, but was: $bytesType_dyn';
      }
      bytesType = bytesType_dyn;
    }

    var anyType_dyn = _data["any_type"];
    dynamic anyType = null;
    if (anyType_dyn != null) {
      if (!(anyType_dyn is dynamic)) {
        throw 'expected dynamic, but was: $anyType_dyn';
      }
      anyType = anyType_dyn;
    }

    var arrayType_dyn = _data["array_type"];
    List<Entry> arrayType = null;
    if (arrayType_dyn != null) {
      if (!(arrayType_dyn is List<dynamic>)) {
        throw 'expected List<dynamic>, but was: $arrayType_dyn';
      }
      arrayType = List.of((arrayType_dyn as List<dynamic>).map((e) => Entry.decode(e)));
    }

    var arrayOfArrayType_dyn = _data["array_of_array_type"];
    List<List<Entry>> arrayOfArrayType = null;
    if (arrayOfArrayType_dyn != null) {
      if (!(arrayOfArrayType_dyn is List<dynamic>)) {
        throw 'expected List<dynamic>, but was: $arrayOfArrayType_dyn';
      }
      arrayOfArrayType = List.of((arrayOfArrayType_dyn as List<dynamic>).map((e) {
        if (!(e is List<dynamic>)) {
          throw 'expected List<dynamic>, but was: $e';
        }
        return List.of((e as List<dynamic>).map((e) => Entry.decode(e)));
      }));
    }

    var mapType_dyn = _data["map_type"];
    Map<String, Entry> mapType = null;
    if (mapType_dyn != null) {
      if (!(mapType_dyn is Map<String, dynamic>)) {
        throw 'expected Map<String, dynamic>, but was: $mapType_dyn';
      }
      mapType = Map.fromEntries((mapType_dyn as Map<String, dynamic>).entries.map((e) => MapEntry(e.key, Entry.decode(e.value))));
    }

    return Entry(booleanType, stringType, datetimeType, unsigned32, unsigned64, signed32, signed64, floatType, doubleType, bytesType, anyType, arrayType, arrayOfArrayType, mapType);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.booleanType != null) {
      _data["boolean_type"] = this.booleanType;
    }

    if (this.stringType != null) {
      _data["string_type"] = this.stringType;
    }

    if (this.datetimeType != null) {
      _data["datetime_type"] = this.datetimeType;
    }

    if (this.unsigned32 != null) {
      _data["unsigned_32"] = this.unsigned32;
    }

    if (this.unsigned64 != null) {
      _data["unsigned_64"] = this.unsigned64;
    }

    if (this.signed32 != null) {
      _data["signed_32"] = this.signed32;
    }

    if (this.signed64 != null) {
      _data["signed_64"] = this.signed64;
    }

    if (this.floatType != null) {
      _data["float_type"] = this.floatType;
    }

    if (this.doubleType != null) {
      _data["double_type"] = this.doubleType;
    }

    if (this.bytesType != null) {
      _data["bytes_type"] = this.bytesType;
    }

    if (this.anyType != null) {
      _data["any_type"] = this.anyType;
    }

    if (this.arrayType != null) {
      _data["array_type"] = List.from(this.arrayType.map((e) => e.encode()));
    }

    if (this.arrayOfArrayType != null) {
      _data["array_of_array_type"] = List.from(this.arrayOfArrayType.map((e) => List.from(e.map((e) => e.encode()))));
    }

    if (this.mapType != null) {
      _data["map_type"] = Map.fromEntries(this.mapType.entries.map((e) => MapEntry(e.key, e.value.encode())));
    }

    return _data;
  }
}
