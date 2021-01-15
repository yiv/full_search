/// bindings for `libsearch`

import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart' as ffi;

// ignore_for_file: unused_import, camel_case_types, non_constant_identifier_names
final DynamicLibrary _dl = _open();
/// Reference to the Dynamic Library, it should be only used for low-level access
final DynamicLibrary dl = _dl;
DynamicLibrary _open() {
  if (Platform.isAndroid) return DynamicLibrary.open('libsearch_ffi.so');
  if (Platform.isIOS) return DynamicLibrary.executable();
  throw UnsupportedError('This platform is not supported.');
}

/// C function `add`.
int add(
  int a,
  int b,
) {
  return _add(a, b);
}
final _add_Dart _add = _dl.lookupFunction<_add_C, _add_Dart>('add');
typedef _add_C = Int64 Function(
  Int64 a,
  Int64 b,
);
typedef _add_Dart = int Function(
  int a,
  int b,
);

/// C function `error_message_utf8`.
int error_message_utf8(
  Pointer<ffi.Utf8> buf,
  int length,
) {
  return _error_message_utf8(buf, length);
}
final _error_message_utf8_Dart _error_message_utf8 = _dl.lookupFunction<_error_message_utf8_C, _error_message_utf8_Dart>('error_message_utf8');
typedef _error_message_utf8_C = Int32 Function(
  Pointer<ffi.Utf8> buf,
  Int32 length,
);
typedef _error_message_utf8_Dart = int Function(
  Pointer<ffi.Utf8> buf,
  int length,
);

/// C function `last_error_length`.
int last_error_length() {
  return _last_error_length();
}
final _last_error_length_Dart _last_error_length = _dl.lookupFunction<_last_error_length_C, _last_error_length_Dart>('last_error_length');
typedef _last_error_length_C = Int32 Function();
typedef _last_error_length_Dart = int Function();

/// C function `se_delete_by_str`.
int se_delete_by_str(
  int port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
) {
  return _se_delete_by_str(port, field, value);
}
final _se_delete_by_str_Dart _se_delete_by_str = _dl.lookupFunction<_se_delete_by_str_C, _se_delete_by_str_Dart>('se_delete_by_str');
typedef _se_delete_by_str_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
);
typedef _se_delete_by_str_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
);

/// C function `se_delete_by_u64`.
int se_delete_by_u64(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
) {
  return _se_delete_by_u64(port, field, value);
}
final _se_delete_by_u64_Dart _se_delete_by_u64 = _dl.lookupFunction<_se_delete_by_u64_C, _se_delete_by_u64_Dart>('se_delete_by_u64');
typedef _se_delete_by_u64_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Uint64 value,
);
typedef _se_delete_by_u64_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
);

/// C function `se_exists`.
int se_exists(
  int port,
) {
  return _se_exists(port);
}
final _se_exists_Dart _se_exists = _dl.lookupFunction<_se_exists_C, _se_exists_Dart>('se_exists');
typedef _se_exists_C = Int32 Function(
  Int64 port,
);
typedef _se_exists_Dart = int Function(
  int port,
);

/// C function `se_get_by_i64`.
int se_get_by_i64(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
) {
  return _se_get_by_i64(port, field, value);
}
final _se_get_by_i64_Dart _se_get_by_i64 = _dl.lookupFunction<_se_get_by_i64_C, _se_get_by_i64_Dart>('se_get_by_i64');
typedef _se_get_by_i64_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Int64 value,
);
typedef _se_get_by_i64_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
);

/// C function `se_get_by_u64`.
int se_get_by_u64(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
) {
  return _se_get_by_u64(port, field, value);
}
final _se_get_by_u64_Dart _se_get_by_u64 = _dl.lookupFunction<_se_get_by_u64_C, _se_get_by_u64_Dart>('se_get_by_u64');
typedef _se_get_by_u64_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Uint64 value,
);
typedef _se_get_by_u64_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
);

/// C function `se_index`.
int se_index(
  int port,
  Pointer<ffi.Utf8> doc,
) {
  return _se_index(port, doc);
}
final _se_index_Dart _se_index = _dl.lookupFunction<_se_index_C, _se_index_Dart>('se_index');
typedef _se_index_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> doc,
);
typedef _se_index_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> doc,
);

/// C function `se_open_or_create`.
int se_open_or_create(
  Pointer<ffi.Utf8> path,
  Pointer<ffi.Utf8> schema,
) {
  return _se_open_or_create(path, schema);
}
final _se_open_or_create_Dart _se_open_or_create = _dl.lookupFunction<_se_open_or_create_C, _se_open_or_create_Dart>('se_open_or_create');
typedef _se_open_or_create_C = Int32 Function(
  Pointer<ffi.Utf8> path,
  Pointer<ffi.Utf8> schema,
);
typedef _se_open_or_create_Dart = int Function(
  Pointer<ffi.Utf8> path,
  Pointer<ffi.Utf8> schema,
);

/// C function `se_search`.
int se_search(
  int port,
  Pointer<ffi.Utf8> query,
  Pointer<ffi.Utf8> fields,
  int page_start,
  int page_size,
) {
  return _se_search(port, query, fields, page_start, page_size);
}
final _se_search_Dart _se_search = _dl.lookupFunction<_se_search_C, _se_search_Dart>('se_search');
typedef _se_search_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> query,
  Pointer<ffi.Utf8> fields,
  Uint32 page_start,
  Uint32 page_size,
);
typedef _se_search_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> query,
  Pointer<ffi.Utf8> fields,
  int page_start,
  int page_size,
);

/// C function `se_update_by_str`.
int se_update_by_str(
  int port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
  Pointer<ffi.Utf8> doc,
) {
  return _se_update_by_str(port, field, value, doc);
}
final _se_update_by_str_Dart _se_update_by_str = _dl.lookupFunction<_se_update_by_str_C, _se_update_by_str_Dart>('se_update_by_str');
typedef _se_update_by_str_C = Int64 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
  Pointer<ffi.Utf8> doc,
);
typedef _se_update_by_str_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  Pointer<ffi.Utf8> value,
  Pointer<ffi.Utf8> doc,
);

/// C function `se_update_by_u64`.
int se_update_by_u64(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
  Pointer<ffi.Utf8> doc,
) {
  return _se_update_by_u64(port, field, value, doc);
}
final _se_update_by_u64_Dart _se_update_by_u64 = _dl.lookupFunction<_se_update_by_u64_C, _se_update_by_u64_Dart>('se_update_by_u64');
typedef _se_update_by_u64_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> field,
  Uint64 value,
  Pointer<ffi.Utf8> doc,
);
typedef _se_update_by_u64_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> field,
  int value,
  Pointer<ffi.Utf8> doc,
);

/// Binding to `allo-isolate` crate
void store_dart_post_cobject(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
) {
  _store_dart_post_cobject(ptr);
}
final _store_dart_post_cobject_Dart _store_dart_post_cobject = _dl.lookupFunction<_store_dart_post_cobject_C, _store_dart_post_cobject_Dart>('store_dart_post_cobject');
typedef _store_dart_post_cobject_C = Void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
