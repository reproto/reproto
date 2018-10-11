class Entry {
  EnumExplicit explicit;
  EnumImplicit implicit;
  EnumU32 enumU32;
  EnumU64 enumU64;
  EnumI32 enumI32;
  EnumI64 enumI64;

  Entry(
    this.explicit,
    this.implicit,
    this.enumU32,
    this.enumU64,
    this.enumI32,
    this.enumI64
  );

  static Entry decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var explicit_dyn = _data["explicit"];
    EnumExplicit explicit = null;
    if (explicit_dyn != null) {
      explicit = EnumExplicit.decode(explicit_dyn);
    }

    var implicit_dyn = _data["implicit"];
    EnumImplicit implicit = null;
    if (implicit_dyn != null) {
      implicit = EnumImplicit.decode(implicit_dyn);
    }

    var enumU32_dyn = _data["enum_u32"];
    EnumU32 enumU32 = null;
    if (enumU32_dyn != null) {
      enumU32 = EnumU32.decode(enumU32_dyn);
    }

    var enumU64_dyn = _data["enum_u64"];
    EnumU64 enumU64 = null;
    if (enumU64_dyn != null) {
      enumU64 = EnumU64.decode(enumU64_dyn);
    }

    var enumI32_dyn = _data["enum_i32"];
    EnumI32 enumI32 = null;
    if (enumI32_dyn != null) {
      enumI32 = EnumI32.decode(enumI32_dyn);
    }

    var enumI64_dyn = _data["enum_i64"];
    EnumI64 enumI64 = null;
    if (enumI64_dyn != null) {
      enumI64 = EnumI64.decode(enumI64_dyn);
    }

    return Entry(explicit, implicit, enumU32, enumU64, enumI32, enumI64);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.explicit != null) {
      _data["explicit"] = this.explicit.encode();
    }

    if (this.implicit != null) {
      _data["implicit"] = this.implicit.encode();
    }

    if (this.enumU32 != null) {
      _data["enum_u32"] = this.enumU32.encode();
    }

    if (this.enumU64 != null) {
      _data["enum_u64"] = this.enumU64.encode();
    }

    if (this.enumI32 != null) {
      _data["enum_i32"] = this.enumI32.encode();
    }

    if (this.enumI64 != null) {
      _data["enum_i64"] = this.enumI64.encode();
    }

    return _data;
  }
}

/// Explicitly assigned strings
class EnumExplicit {
  final _value;
  const EnumExplicit._new(this._value);
  toString() => 'EnumExplicit.$_value';

  static const A = const EnumExplicit._new("foo");
  static const B = const EnumExplicit._new("bar");

  static EnumExplicit decode(dynamic data) {
    if (!(data is String)) {
      throw 'expected String, but got: $data';
    }

    switch (data as String) {
    case "foo":
      return EnumExplicit.A;
    case "bar":
      return EnumExplicit.B;
    default:
      throw 'unexpected EnumExplicit value: $data';
    }
  }

  String encode() {
    return _value;
  }
}

/// Implicit naming depending on the variant
class EnumImplicit {
  final _value;
  const EnumImplicit._new(this._value);
  toString() => 'EnumImplicit.$_value';

  static const A = const EnumImplicit._new("A");
  static const B = const EnumImplicit._new("B");

  static EnumImplicit decode(dynamic data) {
    if (!(data is String)) {
      throw 'expected String, but got: $data';
    }

    switch (data as String) {
    case "A":
      return EnumImplicit.A;
    case "B":
      return EnumImplicit.B;
    default:
      throw 'unexpected EnumImplicit value: $data';
    }
  }

  String encode() {
    return _value;
  }
}

/// Variants with long names.
class EnumLongNames {
  final _value;
  const EnumLongNames._new(this._value);
  toString() => 'EnumLongNames.$_value';

  static const FooBar = const EnumLongNames._new("FooBar");
  static const Baz = const EnumLongNames._new("Baz");

  static EnumLongNames decode(dynamic data) {
    if (!(data is String)) {
      throw 'expected String, but got: $data';
    }

    switch (data as String) {
    case "FooBar":
      return EnumLongNames.FooBar;
    case "Baz":
      return EnumLongNames.Baz;
    default:
      throw 'unexpected EnumLongNames value: $data';
    }
  }

  String encode() {
    return _value;
  }
}

class EnumU32 {
  final _value;
  const EnumU32._new(this._value);
  toString() => 'EnumU32.$_value';

  static const Min = const EnumU32._new(0);
  static const Max = const EnumU32._new(2147483647);

  static EnumU32 decode(dynamic data) {
    if (!(data is int)) {
      throw 'expected int, but got: $data';
    }

    switch (data as int) {
    case 0:
      return EnumU32.Min;
    case 2147483647:
      return EnumU32.Max;
    default:
      throw 'unexpected EnumU32 value: $data';
    }
  }

  int encode() {
    return _value;
  }
}

class EnumU64 {
  final _value;
  const EnumU64._new(this._value);
  toString() => 'EnumU64.$_value';

  static const Min = const EnumU64._new(0);
  static const Max = const EnumU64._new(9007199254740991);

  static EnumU64 decode(dynamic data) {
    if (!(data is int)) {
      throw 'expected int, but got: $data';
    }

    switch (data as int) {
    case 0:
      return EnumU64.Min;
    case 9007199254740991:
      return EnumU64.Max;
    default:
      throw 'unexpected EnumU64 value: $data';
    }
  }

  int encode() {
    return _value;
  }
}

class EnumI32 {
  final _value;
  const EnumI32._new(this._value);
  toString() => 'EnumI32.$_value';

  static const Min = const EnumI32._new(-2147483648);
  static const NegativeOne = const EnumI32._new(-1);
  static const Zero = const EnumI32._new(0);
  static const Max = const EnumI32._new(2147483647);

  static EnumI32 decode(dynamic data) {
    if (!(data is int)) {
      throw 'expected int, but got: $data';
    }

    switch (data as int) {
    case -2147483648:
      return EnumI32.Min;
    case -1:
      return EnumI32.NegativeOne;
    case 0:
      return EnumI32.Zero;
    case 2147483647:
      return EnumI32.Max;
    default:
      throw 'unexpected EnumI32 value: $data';
    }
  }

  int encode() {
    return _value;
  }
}

class EnumI64 {
  final _value;
  const EnumI64._new(this._value);
  toString() => 'EnumI64.$_value';

  static const Min = const EnumI64._new(-9007199254740991);
  static const NegativeOne = const EnumI64._new(-1);
  static const Zero = const EnumI64._new(0);
  static const Max = const EnumI64._new(9007199254740991);

  static EnumI64 decode(dynamic data) {
    if (!(data is int)) {
      throw 'expected int, but got: $data';
    }

    switch (data as int) {
    case -9007199254740991:
      return EnumI64.Min;
    case -1:
      return EnumI64.NegativeOne;
    case 0:
      return EnumI64.Zero;
    case 9007199254740991:
      return EnumI64.Max;
    default:
      throw 'unexpected EnumI64 value: $data';
    }
  }

  int encode() {
    return _value;
  }
}
