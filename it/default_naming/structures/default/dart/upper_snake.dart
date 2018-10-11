class Value {
  String fooBar;

  Value(
    this.fooBar
  );

  static Value decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var fooBar_dyn = _data["FOO_BAR"];
    if (fooBar_dyn == null) {
      throw "expected value but was null";
    }
    if (!(fooBar_dyn is String)) {
      throw 'expected String, but was: $fooBar_dyn';
    }
    final String fooBar = fooBar_dyn;

    return Value(fooBar);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["FOO_BAR"] = this.fooBar;

    return _data;
  }
}
