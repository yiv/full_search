import 'ffi.dart' as native;
import 'package:ffi/ffi.dart';
import 'package:isolate/ports.dart';
import 'dart:ffi';
import 'dart:async';
import 'dart:convert';

class SearchEngine {

  /// setup the engine prerequisities
  static setup() {
    native.store_dart_post_cobject(NativeApi.postCObject);
    print("Scrap Setup Done");
  }


  /// open a exist index or create a new directory to store index files
  /// path: the location path of directory on device
  /// schema: a json format string which defines the field data types of object
  /// for example:
  /// ```
  /// {"message_id": "i64","user_id": "i64", "guild_id": "i64", "channel_id": "i64", "timestamp": "date", "content": "text"}
  /// ```
  int openOrCreate(String path, String schema) {
    final _path = Utf8.toUtf8(path);
    final _schema = Utf8.toUtf8(schema);
    return native.se_open_or_create(_path, _schema);
  }

  /// test whether the engin has been created
  Future<bool> exists() {
    final completer = Completer<bool>();
    final sendPort = singleCompletePort(completer);

    final res = native.se_exists(sendPort.nativePort);
    if (res != 1) {
      _throwError();
    }
    return completer.future;
  }

  /// store and index the object
  /// doc: a json format string of the object which want to be indexed
  Future<void> index(String doc) {
    final _doc = Utf8.toUtf8(doc);
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);

    final res = native.se_index(sendPort.nativePort, _doc);
    if (res != 1) {
      _throwError();
    }
    return completer.future;
  }


  /// query and search the specified fields
  /// query: keywords used for search
  /// fields: search for the content of which fields
  Future<String> search(String query, List fields, int pageStart, int pageSize) {
    final _query = Utf8.toUtf8(query);
    final _fields = Utf8.toUtf8(json.encode(fields));
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);


    final res = native.se_search(sendPort.nativePort, _query, _fields, pageStart, pageSize);
    if (res != 1) {
      print('fields=$fields');
      _throwError();
    }
    return completer.future;
  }

  /// retrive a specify document by giving a field of i64 and it's value
  Future<String> getByI64( String field, int id) {
    final _field = Utf8.toUtf8(field);
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);


    final res = native.se_get_by_i64(sendPort.nativePort, _field, id);
    if (res != 1) {
      print('fields=$field, id=$id');
      _throwError();
    }
    return completer.future;
  }

  /// retrive a specify document by giving a field of u64 and it's value
  Future<String> getByU64( String field, int id) {
    final _field = Utf8.toUtf8(field);
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);


    final res = native.se_get_by_u64(sendPort.nativePort, _field, id);
    if (res != 1) {
      print('fields=$field, id=$id');
      _throwError();
    }
    return completer.future;
  }


  void _throwError() {
    final length = native.last_error_length();
    final Pointer<Utf8> message = allocate(count: length);
    native.error_message_utf8(message, length);
    final error = Utf8.fromUtf8(message);
    print(error);
    throw error;
  }
}
