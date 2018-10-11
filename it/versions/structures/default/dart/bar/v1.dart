class Other {
  String name;

  Other(
    this.name
  );

  static Other decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var name_dyn = _data["name"];
    if (name_dyn == null) {
      throw "expected value but was null";
    }
    if (!(name_dyn is String)) {
      throw 'expected String, but was: $name_dyn';
    }
    final String name = name_dyn;

    return Other(name);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["name"] = this.name;

    return _data;
  }
}
