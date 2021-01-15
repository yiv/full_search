import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:full_search/full_search.dart';
import 'package:full_search_example/search_list.dart';
import 'demo.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  List _result;
  TextEditingController resultController = TextEditingController();

  // int _result = 0;
  String _path = "";
  String _schema = schema;
  String _doc = demo;
  SearchEngine engine;

  @override
  void initState() {
    super.initState();
    initSearchEngine();
    resultController.addListener(_search);
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  void initSearchEngine() async {
    engine = SearchEngine();
    SearchEngine.setup();

    final directory = await getApplicationDocumentsDirectory();
    _path = directory.path;

    engine.openOrCreate(_path, _schema);

    var exists = await engine.exists();
    print('engine exists: $exists');

    var demo = jsonDecode(_doc);
    for (var v in demo) {
      final result = await engine.getByI64('message_id', v['message_id']);
      // print('edwin 50 $result');
      if (result == null) {
        var s = jsonEncode(v);
        engine.index(s);
      }
    }
  }

  void _search() async {
    final keyword = resultController.text;
    final keys = keyword.split(' ');
    var key = '';
    for (var v in keys) {
      if (v == '') {
        continue;
      }
      key = key + ' content:$v';
    }
    print('keyword updating: $key');
    final res = await engine.search(key, ["content"], 1, 10);
    print('edwin 71 $res');
    setState(() {
      _result = (json.decode(res) as List).map((e) => Result.fromJson(e)).toList();
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Full Text Search'),
        ),
        body: Column(
          children: [
            TextField(
                controller: resultController,
                // onChanged: (v) => resultController.text = v,
                decoration: InputDecoration(
                  labelText: 'Name the Pup',
                )),
             Container(child: SearchList(_result)),
          ],
        ),
      ),
    );
  }
}
