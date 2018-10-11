import "foo/v4.dart" as foo;

class Entry {
  foo.Thing thing;

  Entry(
    this.thing
  );

  static Entry decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var thing_dyn = _data["thing"];
    foo.Thing thing = null;
    if (thing_dyn != null) {
      thing = foo.Thing.decode(thing_dyn);
    }

    return Entry(thing);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.thing != null) {
      _data["thing"] = this.thing.encode();
    }

    return _data;
  }
}
