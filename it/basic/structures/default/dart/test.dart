class Entry {
  /// The foo field.
  Foo foo;

  Entry(
    this.foo
  );

  static Entry decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var foo_dyn = _data["foo"];
    Foo foo = null;
    if (foo_dyn != null) {
      foo = Foo.decode(foo_dyn);
    }

    return Entry(foo);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.foo != null) {
      _data["foo"] = this.foo.encode();
    }

    return _data;
  }
}

class Foo {
  /// The field.
  String field;

  Foo(
    this.field
  );

  static Foo decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var field_dyn = _data["field"];
    if (field_dyn == null) {
      throw "expected value but was null";
    }
    if (!(field_dyn is String)) {
      throw 'expected String, but was: $field_dyn';
    }
    final String field = field_dyn;

    return Foo(field);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["field"] = this.field;

    return _data;
  }
}

class Bar {
  /// The inner field.
  Bar_Inner field;

  Bar(
    this.field
  );

  static Bar decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var field_dyn = _data["field"];
    if (field_dyn == null) {
      throw "expected value but was null";
    }
    final Bar_Inner field = Bar_Inner.decode(field_dyn);

    return Bar(field);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["field"] = this.field.encode();

    return _data;
  }
}

class Bar_Inner {
  /// The field.
  String field;

  Bar_Inner(
    this.field
  );

  static Bar_Inner decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var field_dyn = _data["field"];
    if (field_dyn == null) {
      throw "expected value but was null";
    }
    if (!(field_dyn is String)) {
      throw 'expected String, but was: $field_dyn';
    }
    final String field = field_dyn;

    return Bar_Inner(field);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["field"] = this.field;

    return _data;
  }
}