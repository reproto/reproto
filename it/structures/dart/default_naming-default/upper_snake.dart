class Value {
  String fooBar;

  Value(this.fooBar);

  static Value decode(dynamic data) {
    if (!(data is Map<String, dynamic>)) {
      throw "expected Map<String, dynamic> but got $data";
    }

    Map<String, dynamic> _data = data;

    var fooBar_dyn = _data["FOO_BAR"];

    if (fooBar_dyn == null) {
      throw "expected value but was null";
    }

    if (!(fooBar_dyn is String)) {
      throw "expected String, but was: fooBar_dyn";
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
