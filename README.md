# Full Text Search

This is a full text search Flutter plugin which build on Tantivy.

Tantivy is a full-text search engine library inspired by Apache Lucene and written in Rust.

## Schema

The schema describes the type (string, text, u64) of a field as well as how it should be handled.

Schema field type supported:

| Field Type | Description |
|:---|:---|
| string |  String field type configuration|
| text |  Unsigned 64-bits integers field type configuration |
| u64 |  Unsigned 64-bits integers field type configuration|
| i64 |  Signed 64-bits integers 64 field type configuration|
| f64 |  64-bits float 64 field type configuration|
| date | Signed 64-bits Date field type configuration|
| facet |  Hierachical Facet|
| bytes |Bytes (one per document)|

## Nedd to know

* Up to now, one SearchEngine for one kind of data type, it's not the right way. Need to be changed.
* Underground, this plugin build on 'static or shared native library of rust', it uses Dart FFI to call the rust code

## Example


``` dart

/// create a instance of SearchEngine and set it up
final engine = SearchEngine();
SearchEngine.setup();

/// get the path which used to store index files
final _path = await getApplicationDocumentsDirectory().path;

/// define the schema of the data which wanted to be indexed
final _schema = r'{"id": "i64",  "timestamp": "date", "content": "text"}';

/// open a exists one or create a new one on the device
engine.openOrCreate(_path, _schema);

/// encode the data object to a json string
final _doc = jsonEncode(dataObject);
/// start to index and store the data
engine.index(_doc);

/// give the query keywords and the field which to search on 
final res = await engine.search('关键字 关键词', ["content"], 1, 10);

```

![](https://github.com/yiv/full_search/blob/master/example/example.png)



