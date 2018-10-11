import "lower_camel.dart" as lower_camel;
import "lower_snake.dart" as lower_snake;
import "upper_camel.dart" as upper_camel;
import "upper_snake.dart" as upper_snake;

class Entry {
  lower_camel.Value lowerCamel;
  lower_snake.Value lowerSnake;
  upper_camel.Value upperCamel;
  upper_snake.Value upperSnake;

  Entry(
    this.lowerCamel,
    this.lowerSnake,
    this.upperCamel,
    this.upperSnake
  );

  static Entry decode(dynamic _data_dyn) {
    if (!(_data_dyn is Map<String, dynamic>)) {
      throw 'expected Map<String, dynamic>, but got: $_data_dyn';
    }

    Map<String, dynamic> _data = _data_dyn;

    var lowerCamel_dyn = _data["lower_camel"];
    lower_camel.Value lowerCamel = null;
    if (lowerCamel_dyn != null) {
      lowerCamel = lower_camel.Value.decode(lowerCamel_dyn);
    }

    var lowerSnake_dyn = _data["lower_snake"];
    lower_snake.Value lowerSnake = null;
    if (lowerSnake_dyn != null) {
      lowerSnake = lower_snake.Value.decode(lowerSnake_dyn);
    }

    var upperCamel_dyn = _data["upper_camel"];
    upper_camel.Value upperCamel = null;
    if (upperCamel_dyn != null) {
      upperCamel = upper_camel.Value.decode(upperCamel_dyn);
    }

    var upperSnake_dyn = _data["upper_snake"];
    upper_snake.Value upperSnake = null;
    if (upperSnake_dyn != null) {
      upperSnake = upper_snake.Value.decode(upperSnake_dyn);
    }

    return Entry(lowerCamel, lowerSnake, upperCamel, upperSnake);
  }

  Map<String, dynamic> encode() {
    Map<String, dynamic> _data = Map();

    if (this.lowerCamel != null) {
      _data["lower_camel"] = this.lowerCamel.encode();
    }

    if (this.lowerSnake != null) {
      _data["lower_snake"] = this.lowerSnake.encode();
    }

    if (this.upperCamel != null) {
      _data["upper_camel"] = this.upperCamel.encode();
    }

    if (this.upperSnake != null) {
      _data["upper_snake"] = this.upperSnake.encode();
    }

    return _data;
  }
}
