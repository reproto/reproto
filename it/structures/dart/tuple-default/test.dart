class Entry {
  Tuple1 tuple1;
  Tuple2 tuple2;

  Entry(this.tuple1, this.tuple2);

  static Entry decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var tuple1_dyn = _data["tuple1"];

    Tuple1 tuple1 = null;

    if (tuple1_dyn != null) {
      tuple1 = Tuple1.decode(tuple1_dyn);
    }

    var tuple2_dyn = _data["tuple2"];

    Tuple2 tuple2 = null;

    if (tuple2_dyn != null) {
      tuple2 = Tuple2.decode(tuple2_dyn);
    }

    return Entry(tuple1, tuple2);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.tuple1 != null) {
      _data["tuple1"] = this.tuple1.encode();
    }

    if (this.tuple2 != null) {
      _data["tuple2"] = this.tuple2.encode();
    }

    return _data;
  }
}

/// Tuple containing primitive.
class Tuple1 {
  String a;
  int b;

  Tuple1(this.a, this.b);

  static Tuple1 decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 2) {
      throw "expected array of length 2, but was $_data.length";
    }

    var a_dyn = _data[0];

    if (a_dyn == null) {
      throw "expected value but was null";
    }

    if (!(a_dyn is String)) {
      throw "expected String, but was: a_dyn";
    }
    final String a = a_dyn;

    var b_dyn = _data[1];

    if (b_dyn == null) {
      throw "expected value but was null";
    }

    if (!(b_dyn is int)) {
      throw "expected int, but was: b_dyn";
    }
    final int b = b_dyn;

    return Tuple1(a, b);
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    _data.add(this.a);_data.add(this.b);

    return _data;
  }
}

/// Tuple containing object.
class Tuple2 {
  String a;
  Other b;

  Tuple2(this.a, this.b);

  static Tuple2 decode(dynamic data) {
    if (!(data is List<dynamic>)) {
      throw "expected List<dynamic> but got $data";
    }

    List<dynamic> _data = data;

    if (_data.length != 2) {
      throw "expected array of length 2, but was $_data.length";
    }

    var a_dyn = _data[0];

    if (a_dyn == null) {
      throw "expected value but was null";
    }

    if (!(a_dyn is String)) {
      throw "expected String, but was: a_dyn";
    }
    final String a = a_dyn;

    var b_dyn = _data[1];

    if (b_dyn == null) {
      throw "expected value but was null";
    }

    final Other b = Other.decode(b_dyn);

    return Tuple2(a, b);
  }

  List<dynamic> encode() {
    List<dynamic> _data = List();

    _data.add(this.a);_data.add(this.b.encode());

    return _data;
  }
}

/// Complex object.
class Other {
  String a;

  Other(this.a);

  static Other decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var a_dyn = _data["a"];

    if (a_dyn == null) {
      throw "expected value but was null";
    }

    if (!(a_dyn is String)) {
      throw "expected String, but was: a_dyn";
    }
    final String a = a_dyn;

    return Other(a);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["a"] = this.a;

    return _data;
  }
}
