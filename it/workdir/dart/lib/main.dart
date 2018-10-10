import 'dart:async' show Future;
import 'dart:convert';
import 'dart:io';
import 'dart:convert';
import 'test.dart' as test;

Future<void> main() async {
    var lines = stdin.transform(Utf8Decoder()).transform(LineSplitter());

    await for (var line in lines) {
        var data = jsonDecode(line.trim());
        var entry = test.Entry.decode(data);
        print(jsonEncode(entry.encode()));
    }
}
