class Other {
  String name21;

  Other(
    this.name21
  );

  static Other decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var name21_dyn = _data["name21"];
    if (name21_dyn == null) {
      throw "expected value but was null";
    }
    if (!(name21_dyn is String)) {
      throw 'expected String, but was: $name21_dyn';
    }
    final String name21 = name21_dyn;

    return Other(name21);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["name21"] = this.name21;

    return _data;
  }
}
