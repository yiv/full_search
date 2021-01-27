#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate ffi_helpers;
#[macro_use]
extern crate serde_derive;

use std::{
    ffi::{CStr, CString},
    io,
    os::raw::c_char,
};

use allo_isolate::{store_dart_post_cobject, Isolate};
use ffi_helpers::{null_pointer_check, update_last_error};
use frusty_logger::{Config, FilterBuilder};
use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

use std::borrow::Borrow;

#[macro_use]
mod helper;

use helper::*;

mod demo;
mod error;

pub static RUNTIME: Lazy<io::Result<Runtime>> = Lazy::new(|| {
    Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("flutterust")
        .build()
});

frusty_logger::include_ffi!(
  with_config: Config::new(
    log::Level::Debug,
    FilterBuilder::new()
    .parse("search_ffi,search")
    .build()
  )
);

#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[no_mangle]
pub extern "C" fn se_open_or_create(path: *const c_char, schema: *const c_char) -> i32 {
    let path = cstr!(path);
    let schema = cstr!(schema);

    log::debug!("open path={}, schema={}", path, schema);
    match search::open(path, schema) {
        Ok(v) => 1,
        Err(err) => {
            log::error!("{}", err.to_string());
            last_err(err)
        }
    }
}

#[no_mangle]
pub extern "C" fn se_exists(port: i64) -> i32 {
    let rt = runtime!();

    log::debug!("exists ");
    let task = search::exists();
    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_index(port: i64, doc: *const c_char) -> i32 {
    let rt = runtime!();
    let doc = cstr!(doc);

    log::debug!("index ");
    let task = search::index(doc);
    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_search(
    port: i64,
    query: *const c_char,
    fields: *const c_char,
    page_start: u32,
    page_size: u32,
) -> i32 {
    let rt = runtime!();
    let query = cstr!(query);
    let fields = cstr!(fields);

    log::debug!("search query={}, fields={} ", query, fields);
    let fields = match serde_json::from_str::<Vec<String>>(&fields) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err.to_string());
            update_last_error(err);
            return 0;
        }
    };

    let task = search::search(query, fields, page_start as usize, page_size as usize);
    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_delete_by_str(port: i64, field: *const c_char, value: *const c_char) -> i32 {
    let rt = runtime!();
    let field = cstr!(field);
    let value = cstr!(value);

    log::debug!("delete_by_str field={}, value={} ", field, value);

    let task = search::delete_by_str(field, value);
    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_delete_by_u64(port: i64, field: *const c_char, value: u64) -> i32 {
    let rt = runtime!();
    let field = cstr!(field);

    log::debug!("delete_by_u64 field={}, value={} ", field, value);

    let task = search::delete_by_u64(field, value);
    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_update_by_str(
    port: i64,
    field: *const c_char,
    value: *const c_char,
    doc: *const c_char,
) -> i64 {
    let rt = runtime!();
    let field = cstr!(field);
    let doc = cstr!(doc);
    let value = cstr!(value);

    log::debug!("update_by_str field={}, value={} ", field, value);

    let task = search::update_by_str(field, value, doc);

    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_update_by_u64(
    port: i64,
    field: *const c_char,
    value: u64,
    doc: *const c_char,
) -> i32 {
    let rt = runtime!();
    let field = cstr!(field);
    let doc = cstr!(doc);

    let task = search::update_by_u64(&field, value, &doc);

    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_get_by_u64(port: i64, field: *const c_char, value: u64) -> i32 {
    let rt = runtime!();
    let field = cstr!(field);

    let task = search::get_by_term_u64(&field, value);

    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[no_mangle]
pub extern "C" fn se_get_by_i64(port: i64, field: *const c_char, value: i64) -> i32 {
    let rt = runtime!();
    let field = cstr!(field);

    let task = search::get_by_term_i64(&field, value);

    let t = Isolate::new(port).task(task);
    rt.spawn(t);
    1
}

#[tokio::test]
async fn test() {
    search::open("./db", demo::DEMO_SCHEMA).unwrap();

    {
        // let now = std::time::Instant::now();
        // let data = serde_json::from_str::<Vec<demo::DemoMessage>>(demo::DEMO_DATA).unwrap();
        // for v in data {
        //     let s = serde_json::to_string(&v).unwrap();
        //     search::index(&s).await.unwrap()
        // }
        // println!("{}", now.elapsed().as_millis());
    }

    // let res = search::search(r#"content:儿子 AND message_id:[139259496311685120 TO 1392594963116851200}"#, vec!["content".to_string()], 1, 10).await.unwrap();
    // let res = search::search(r#"message_id:141906710238461952"#, vec!["content".to_string(), "message_id".to_string(), "id".to_string()], 1, 10).await.unwrap();
    // let res = search::search(r#"message_id:[5 TO 6]"#, vec!["content".to_string(), "message_id".to_string(), "id".to_string()], 1, 10).await.unwrap();
    // let res = search::search(r#"(content:儿子 OR content:亲 ) AND message_id:[2 TO 8}"#, vec!["content".to_string(), "message_id".to_string(), "id".to_string()], 1, 10).await.unwrap();
    // let res = search::search(r#"content:儿子 OR content:亲"#, vec!["content".to_string()], 1, 10).await.unwrap();
    // let res = search::search_field("路 痴", &vec!["content".to_string()], 1, 10).await.unwrap();
    search::delete_by_str("id", "104").await.unwrap();
    let res = search::search("104", vec!["id".to_string()], 1, 10)
        .await
        .unwrap();

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    struct ResultItem {
        id: String,
        channel_id: i64,
        user_id: i64,
        guild_id: i64,
        message_id: i64,
        content: String,
        timestamp: String,
    }
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    struct SearchResult {
        result: ResultItem,
        snippet: String,
    }

    let res = serde_json::from_str::<Vec<SearchResult>>(&res).unwrap();

    for v in res.iter() {
        println!("{} - {:?}", v.snippet, v.result);
    }

    println!("found items: {}", res.len());
}
