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
