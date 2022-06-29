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

class RootType {

  RootType();

  static RootType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class RootInterface {
  static RootInterface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "Foo":
        return RootInterface_Foo.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class RootInterface_Foo extends RootInterface {

  RootInterface_Foo();

  static RootInterface_Foo decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "Foo";

    return _data;
  }
}

class RootEnum {
  final _value;
  const RootEnum._new(this._value);

  toString() => "RootEnum._value";

  static const Foo = const RootEnum._new("Foo");

  static RootEnum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Foo":
        return RootEnum.Foo;
      default:
        throw "unexpected RootEnum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class RootTuple {

  RootTuple();

  static RootTuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return RootTuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}

class RootType_NestedType {

  RootType_NestedType();

  static RootType_NestedType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType_NestedType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class RootType_NestedInterface {
  static RootType_NestedInterface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "Foo":
        return RootType_NestedInterface_Foo.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class RootType_NestedInterface_Foo extends RootType_NestedInterface {

  RootType_NestedInterface_Foo();

  static RootType_NestedInterface_Foo decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType_NestedInterface_Foo();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "Foo";

    return _data;
  }
}

class RootType_NestedEnum {
  final _value;
  const RootType_NestedEnum._new(this._value);

  toString() => "RootType_NestedEnum._value";

  static const Foo = const RootType_NestedEnum._new("Foo");

  static RootType_NestedEnum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Foo":
        return RootType_NestedEnum.Foo;
      default:
        throw "unexpected RootType_NestedEnum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class RootType_NestedTuple {

  RootType_NestedTuple();

  static RootType_NestedTuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return RootType_NestedTuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}

class RootInterface_Foo_NestedType {

  RootInterface_Foo_NestedType();

  static RootInterface_Foo_NestedType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo_NestedType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class RootInterface_Foo_NestedInterface {
  static RootInterface_Foo_NestedInterface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "NestedFoo":
        return RootInterface_Foo_NestedInterface_NestedFoo.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class RootInterface_Foo_NestedInterface_NestedFoo extends RootInterface_Foo_NestedInterface {

  RootInterface_Foo_NestedInterface_NestedFoo();

  static RootInterface_Foo_NestedInterface_NestedFoo decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo_NestedInterface_NestedFoo();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "NestedFoo";

    return _data;
  }
}

class RootInterface_Foo_NestedEnum {
  final _value;
  const RootInterface_Foo_NestedEnum._new(this._value);

  toString() => "RootInterface_Foo_NestedEnum._value";

  static const Foo = const RootInterface_Foo_NestedEnum._new("Foo");

  static RootInterface_Foo_NestedEnum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Foo":
        return RootInterface_Foo_NestedEnum.Foo;
      default:
        throw "unexpected RootInterface_Foo_NestedEnum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class RootInterface_Foo_NestedTuple {

  RootInterface_Foo_NestedTuple();

  static RootInterface_Foo_NestedTuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return RootInterface_Foo_NestedTuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}

class RootTuple_NestedType {

  RootTuple_NestedType();

  static RootTuple_NestedType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootTuple_NestedType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class RootTuple_NestedInterface {
  static RootTuple_NestedInterface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "Foo":
        return RootTuple_NestedInterface_Foo.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class RootTuple_NestedInterface_Foo extends RootTuple_NestedInterface {

  RootTuple_NestedInterface_Foo();

  static RootTuple_NestedInterface_Foo decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootTuple_NestedInterface_Foo();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "Foo";

    return _data;
  }
}

class RootTuple_NestedEnum {
  final _value;
  const RootTuple_NestedEnum._new(this._value);

  toString() => "RootTuple_NestedEnum._value";

  static const Foo = const RootTuple_NestedEnum._new("Foo");

  static RootTuple_NestedEnum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Foo":
        return RootTuple_NestedEnum.Foo;
      default:
        throw "unexpected RootTuple_NestedEnum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class RootTuple_NestedTuple {

  RootTuple_NestedTuple();

  static RootTuple_NestedTuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return RootTuple_NestedTuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}

class RootService_NestedType {

  RootService_NestedType();

  static RootService_NestedType decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootService_NestedType();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

abstract class RootService_NestedInterface {
  static RootService_NestedInterface decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tag = _data["type"];

    switch (tag) {
      case "Foo":
        return RootService_NestedInterface_Foo.decode(_data);
      default:
        throw "bad tag: $tag";
    }
  }

  Map<String, dynamic> encode();
}

class RootService_NestedInterface_Foo extends RootService_NestedInterface {

  RootService_NestedInterface_Foo();

  static RootService_NestedInterface_Foo decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootService_NestedInterface_Foo();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["type"] = "Foo";

    return _data;
  }
}

class RootService_NestedEnum {
  final _value;
  const RootService_NestedEnum._new(this._value);

  toString() => "RootService_NestedEnum._value";

  static const Foo = const RootService_NestedEnum._new("Foo");

  static RootService_NestedEnum decode(dynamic data) {
    if (!(data is String)) {
      throw "expected String but got $data";
    }

    switch (data as String) {
      case "Foo":
        return RootService_NestedEnum.Foo;
      default:
        throw "unexpected RootService_NestedEnum value: $data";
    }
  }

  String encode() {
    return _value;
  }
}

class RootService_NestedTuple {

  RootService_NestedTuple();

  static RootService_NestedTuple decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 0) {
      throw "expected array of length 0, but was $_data.length";
    }

    return RootService_NestedTuple();
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    return _data;
  }
}

class RootType_NestedInterface_Foo_Nested {

  RootType_NestedInterface_Foo_Nested();

  static RootType_NestedInterface_Foo_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType_NestedInterface_Foo_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootType_NestedTuple_Nested {

  RootType_NestedTuple_Nested();

  static RootType_NestedTuple_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType_NestedTuple_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootType_NestedService_Nested {

  RootType_NestedService_Nested();

  static RootType_NestedService_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootType_NestedService_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootInterface_Foo_NestedInterface_NestedFoo_Nested {

  RootInterface_Foo_NestedInterface_NestedFoo_Nested();

  static RootInterface_Foo_NestedInterface_NestedFoo_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo_NestedInterface_NestedFoo_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootInterface_Foo_NestedTuple_Nested {

  RootInterface_Foo_NestedTuple_Nested();

  static RootInterface_Foo_NestedTuple_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo_NestedTuple_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootInterface_Foo_NestedService_Nested {

  RootInterface_Foo_NestedService_Nested();

  static RootInterface_Foo_NestedService_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootInterface_Foo_NestedService_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootTuple_NestedInterface_Foo_Nested {

  RootTuple_NestedInterface_Foo_Nested();

  static RootTuple_NestedInterface_Foo_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootTuple_NestedInterface_Foo_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootTuple_NestedTuple_Nested {

  RootTuple_NestedTuple_Nested();

  static RootTuple_NestedTuple_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootTuple_NestedTuple_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootTuple_NestedService_Nested {

  RootTuple_NestedService_Nested();

  static RootTuple_NestedService_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootTuple_NestedService_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootService_NestedInterface_Foo_Nested {

  RootService_NestedInterface_Foo_Nested();

  static RootService_NestedInterface_Foo_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootService_NestedInterface_Foo_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootService_NestedTuple_Nested {

  RootService_NestedTuple_Nested();

  static RootService_NestedTuple_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootService_NestedTuple_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}

class RootService_NestedService_Nested {

  RootService_NestedService_Nested();

  static RootService_NestedService_Nested decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    return RootService_NestedService_Nested();
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    return _data;
  }
}
