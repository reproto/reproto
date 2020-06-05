class Entry {
  A a;
  A_B b;

  Entry(
    this.a,
    this.b
  );

  static Entry decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var a_dyn = _data["a"];
    A a = null;
    if (a_dyn != null) {
      a = A.decode(a_dyn);
    }

    var b_dyn = _data["b"];
    A_B b = null;
    if (b_dyn != null) {
      b = A_B.decode(b_dyn);
    }

    return Entry(a, b);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.a != null) {
      _data["a"] = this.a.encode();
    }

    if (this.b != null) {
      _data["b"] = this.b.encode();
    }

    return _data;
  }
}

class A {
  A_B b;

  A(
    this.b
  );

  static A decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var b_dyn = _data["b"];
    if (b_dyn == null) {
      throw "expected value but was null";
    }
    final A_B b = A_B.decode(b_dyn);

    return A(b);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["b"] = this.b.encode();

    return _data;
  }
}

class A_B {
  String field;

  A_B(
    this.field
  );

  static A_B decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var field_dyn = _data["field"];
    if (field_dyn == null) {
      throw "expected value but was null";
    }
    if (!(field_dyn is String)) {
      throw 'expected String, but was: $field_dyn';
    }
    final String field = field_dyn;

    return A_B(field);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["field"] = this.field;

    return _data;
  }
}
