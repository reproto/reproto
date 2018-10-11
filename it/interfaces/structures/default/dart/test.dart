class Entry {
  Tagged tagged;
  Untagged untagged;

  Entry(
    this.tagged,
    this.untagged
  );

  static Entry decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var tagged_dyn = _data["tagged"];
    Tagged tagged = null;
    if (tagged_dyn != null) {
      tagged = Tagged.decode(tagged_dyn);
    }

    var untagged_dyn = _data["untagged"];
    Untagged untagged = null;
    if (untagged_dyn != null) {
      untagged = Untagged.decode(untagged_dyn);
    }

    return Entry(tagged, untagged);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.tagged != null) {
      _data["tagged"] = this.tagged.encode();
    }

    if (this.untagged != null) {
      _data["untagged"] = this.untagged.encode();
    }

    return _data;
  }
}

abstract class Tagged {
  static Tagged decode(dynamic _dataDyn) {
  if (!(_dataDyn is Map<String, dynamic>)) {
    throw 'expected Map<String, dynamic>, but got: $_dataDyn';
  }
  Map<String, dynamic> _data = _dataDyn;

  var tag = _data["@type"];

  switch (tag) {
  case "foo":
    return Tagged_A.decode(_data);
  case "b":
    return Tagged_B.decode(_data);
  case "Bar":
    return Tagged_Bar.decode(_data);
  case "Baz":
    return Tagged_Baz.decode(_data);
  default:
    throw 'bad tag: $tag';
  }
  }

  Map<String, dynamic> encode();
}

class Tagged_A extends Tagged {
  String shared;

  Tagged_A(
    this.shared
  );

  static Tagged_A decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    return Tagged_A(shared);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["@type"] = "foo";

    _data["shared"] = this.shared;

    return _data;
  }
}

class Tagged_B extends Tagged {
  String shared;

  Tagged_B(
    this.shared
  );

  static Tagged_B decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    return Tagged_B(shared);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["@type"] = "b";

    _data["shared"] = this.shared;

    return _data;
  }
}

class Tagged_Bar extends Tagged {
  String shared;

  Tagged_Bar(
    this.shared
  );

  static Tagged_Bar decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    return Tagged_Bar(shared);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["@type"] = "Bar";

    _data["shared"] = this.shared;

    return _data;
  }
}

class Tagged_Baz extends Tagged {
  String shared;

  Tagged_Baz(
    this.shared
  );

  static Tagged_Baz decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    return Tagged_Baz(shared);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["@type"] = "Baz";

    _data["shared"] = this.shared;

    return _data;
  }
}

abstract class Untagged {
  static Untagged decode(dynamic _dataDyn) {
  if (!(_dataDyn is Map<String, dynamic>)) {
    throw 'expected Map<String, dynamic>, but got: $_dataDyn';
  }
  Map<String, dynamic> _data = _dataDyn;

  var keys = Set.of(_data.keys);

  if (keys.containsAll(<String>["a", "b"])) {
    return Untagged_A.decode(_data);
  }

  if (keys.containsAll(<String>["a"])) {
    return Untagged_B.decode(_data);
  }

  if (keys.containsAll(<String>["b"])) {
    return Untagged_C.decode(_data);
  }
  }

  Map<String, dynamic> encode();
}

/// Special case: fields shared with other sub-types.
/// NOTE: due to rust support through untagged, the types are matched in-order.
class Untagged_A extends Untagged {
  String shared;
  String sharedIgnore;
  String a;
  String b;
  String ignore;

  Untagged_A(
    this.shared,
    this.sharedIgnore,
    this.a,
    this.b,
    this.ignore
  );

  static Untagged_A decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    var sharedIgnore_dyn = _data["shared_ignore"];
    String sharedIgnore = null;
    if (sharedIgnore_dyn != null) {
      if (!(sharedIgnore_dyn is String)) {
        throw 'expected String, but was: $sharedIgnore_dyn';
      }
      sharedIgnore = sharedIgnore_dyn;
    }

    var a_dyn = _data["a"];
    if (a_dyn == null) {
      throw "expected value but was null";
    }
    if (!(a_dyn is String)) {
      throw 'expected String, but was: $a_dyn';
    }
    final String a = a_dyn;

    var b_dyn = _data["b"];
    if (b_dyn == null) {
      throw "expected value but was null";
    }
    if (!(b_dyn is String)) {
      throw 'expected String, but was: $b_dyn';
    }
    final String b = b_dyn;

    var ignore_dyn = _data["ignore"];
    String ignore = null;
    if (ignore_dyn != null) {
      if (!(ignore_dyn is String)) {
        throw 'expected String, but was: $ignore_dyn';
      }
      ignore = ignore_dyn;
    }

    return Untagged_A(shared, sharedIgnore, a, b, ignore);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["shared"] = this.shared;

    if (this.sharedIgnore != null) {
      _data["shared_ignore"] = this.sharedIgnore;
    }

    _data["a"] = this.a;

    _data["b"] = this.b;

    if (this.ignore != null) {
      _data["ignore"] = this.ignore;
    }

    return _data;
  }
}

class Untagged_B extends Untagged {
  String shared;
  String sharedIgnore;
  String a;
  String ignore;

  Untagged_B(
    this.shared,
    this.sharedIgnore,
    this.a,
    this.ignore
  );

  static Untagged_B decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    var sharedIgnore_dyn = _data["shared_ignore"];
    String sharedIgnore = null;
    if (sharedIgnore_dyn != null) {
      if (!(sharedIgnore_dyn is String)) {
        throw 'expected String, but was: $sharedIgnore_dyn';
      }
      sharedIgnore = sharedIgnore_dyn;
    }

    var a_dyn = _data["a"];
    if (a_dyn == null) {
      throw "expected value but was null";
    }
    if (!(a_dyn is String)) {
      throw 'expected String, but was: $a_dyn';
    }
    final String a = a_dyn;

    var ignore_dyn = _data["ignore"];
    String ignore = null;
    if (ignore_dyn != null) {
      if (!(ignore_dyn is String)) {
        throw 'expected String, but was: $ignore_dyn';
      }
      ignore = ignore_dyn;
    }

    return Untagged_B(shared, sharedIgnore, a, ignore);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["shared"] = this.shared;

    if (this.sharedIgnore != null) {
      _data["shared_ignore"] = this.sharedIgnore;
    }

    _data["a"] = this.a;

    if (this.ignore != null) {
      _data["ignore"] = this.ignore;
    }

    return _data;
  }
}

class Untagged_C extends Untagged {
  String shared;
  String sharedIgnore;
  String b;
  String ignore;

  Untagged_C(
    this.shared,
    this.sharedIgnore,
    this.b,
    this.ignore
  );

  static Untagged_C decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var shared_dyn = _data["shared"];
    if (shared_dyn == null) {
      throw "expected value but was null";
    }
    if (!(shared_dyn is String)) {
      throw 'expected String, but was: $shared_dyn';
    }
    final String shared = shared_dyn;

    var sharedIgnore_dyn = _data["shared_ignore"];
    String sharedIgnore = null;
    if (sharedIgnore_dyn != null) {
      if (!(sharedIgnore_dyn is String)) {
        throw 'expected String, but was: $sharedIgnore_dyn';
      }
      sharedIgnore = sharedIgnore_dyn;
    }

    var b_dyn = _data["b"];
    if (b_dyn == null) {
      throw "expected value but was null";
    }
    if (!(b_dyn is String)) {
      throw 'expected String, but was: $b_dyn';
    }
    final String b = b_dyn;

    var ignore_dyn = _data["ignore"];
    String ignore = null;
    if (ignore_dyn != null) {
      if (!(ignore_dyn is String)) {
        throw 'expected String, but was: $ignore_dyn';
      }
      ignore = ignore_dyn;
    }

    return Untagged_C(shared, sharedIgnore, b, ignore);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    _data["shared"] = this.shared;

    if (this.sharedIgnore != null) {
      _data["shared_ignore"] = this.sharedIgnore;
    }

    _data["b"] = this.b;

    if (this.ignore != null) {
      _data["ignore"] = this.ignore;
    }

    return _data;
  }
}
