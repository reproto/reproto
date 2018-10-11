import "../bar/v1.dart" as bar;
import "../bar/v2_0.dart" as bar2;
import "../bar/v2_1.dart" as bar21;

class Thing {
  String name;
  bar.Other other;
  bar2.Other other2;
  bar21.Other other21;

  Thing(
    this.name,
    this.other,
    this.other2,
    this.other21
  );

  static Thing decode(dynamic _dataDyn) {
    if (!(_dataDyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_dataDyn';
    }

    Map<String, dynamic> _data = _dataDyn;

    var name_dyn = _data["name"];
    String name = null;
    if (name_dyn != null) {
      if (!(name_dyn is String)) {
        throw 'expected String, but was: $name_dyn';
      }
      name = name_dyn;
    }

    var other_dyn = _data["other"];
    bar.Other other = null;
    if (other_dyn != null) {
      other = bar.Other.decode(other_dyn);
    }

    var other2_dyn = _data["other2"];
    bar2.Other other2 = null;
    if (other2_dyn != null) {
      other2 = bar2.Other.decode(other2_dyn);
    }

    var other21_dyn = _data["other21"];
    bar21.Other other21 = null;
    if (other21_dyn != null) {
      other21 = bar21.Other.decode(other21_dyn);
    }

    return Thing(name, other, other2, other21);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.name != null) {
      _data["name"] = this.name;
    }

    if (this.other != null) {
      _data["other"] = this.other.encode();
    }

    if (this.other2 != null) {
      _data["other2"] = this.other2.encode();
    }

    if (this.other21 != null) {
      _data["other21"] = this.other21.encode();
    }

    return _data;
  }
}
