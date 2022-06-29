class Entry {

  Entry();

  static Entry decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return Entry();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class Type {

  Type();

  static Type decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return Type();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class Interface {
  static Interface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "SubType":
        return Interface_SubType.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class Interface_SubType extends Interface {

  Interface_SubType();

  static Interface_SubType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return Interface_SubType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "SubType";

    return _data;
  }
}

class Enum {
  final _value;
  const Enum._new(this._value);

  toString() => "Enum._value";

  static const Variant = const Enum._new("Variant");

  static Enum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Variant":
        return Enum.Variant;
      default:
        throw "unexpected Enum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class Tuple {

  Tuple();

  static Tuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return Tuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}
