import '../generated/io/reproto/example.dart';
import 'dart:convert';

void main() {
    var data = jsonDecode('{"name": "John Smith", "age": 42}');
    var person = Person.decode(data);
    print(person.name);
    print(person.age);
    print(jsonEncode(person.encode()));
}
