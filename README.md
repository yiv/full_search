# Full Text Search

This is a full text search Flutter plugin which build on Tantivy.

Tantivy is a full-text search engine library inspired by Apache Lucene and written in Rust.

## Schema

The schema describes the type (string, text, u64) of a field as well as how it should be handled.

Schema field type supported:

| Field Type | Description |
|:---|:---|
| string |  String field type configuration|
| text |  Text field type configuration |
| u64 |  Unsigned 64-bits integers field type configuration|
| i64 |  Signed 64-bits integers 64 field type configuration|
| f64 |  64-bits float 64 field type configuration|
| date | Signed 64-bits Date field type configuration|
| facet |  Hierachical Facet|
| bytes |Bytes (one per document)|

## Query Terms

> Operators like *AND*, *OR*, *TO* **MUST BE** in *UPPERCASE*

* **simple terms**: "e.g.: Barack Obama are simply tokenized using tantivy's SimpleTokenizer, hence becoming ["barack", "obama"]. The terms are then searched within the default terms of the query parser.

  e.g. If body and title are default fields, our example terms are ["title:barack", "body:barack", "title:obama", "body:obama"]. By default, all tokenized and indexed fields are default fields.

* **multiple terms** are handled as an OR : any document containing at least one of the term will go through the scoring.

  This behavior is slower, but is not a bad idea if the user is sorting by relevance : The user typically just scans through the first few documents in order of decreasing relevance and will stop when the documents are not relevant anymore.


* **boolean operators**: AND, OR. AND takes precedence over OR, so that a AND b OR c is interpreted as (a AND b) OR c.

* **In addition to the boolean operators**, the -, + can help define. These operators are sufficient to express all queries using boolean operators. For instance x AND y OR z can be written ((+x +y) z). In addition, these operators can help define "required optional" queries. (+x y) matches the same document set as simply x, but y will help refining the score.

* **negative terms**: By prepending a term by a -, a term can be excluded from the search. This is useful for disambiguating a query. e.g. apple -fruit

* **must terms**: By prepending a term by a +, a term can be made required for the search.

* **phrase terms**: Quoted terms become phrase searches on fields that have positions indexed. e.g., title:"Barack Obama" will only find documents that have "barack" immediately followed by "obama".

* **range terms**: Range searches can be done by specifying the start and end bound. These can be inclusive or exclusive. e.g., title:[a TO c} will find all documents whose title contains a word lexicographically between a and c (inclusive lower bound, exclusive upper bound). Inclusive bounds are [], exclusive are {}.

* **date values**: The query parser supports rfc3339 formatted dates. For example "2002-10-02T15:00:00.05Z"

* **all docs query**: A plain * will match all documents in the index.


## Need to know

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
await engine.index(_doc);

/// give the query keywords and the field which to search on 
final res = await engine.search('关键字 关键词', ['content'], 1, 10);

/// remove a specify document by giving a field of u64 and it's value
await engine.deleteByU64('id', 141906710246850560);

```

![](https://github.com/yiv/full_search/blob/master/example/example.png)



