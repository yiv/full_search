#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use anyhow::{Result, Error};
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, TermQuery, Query};
use tantivy::tokenizer::*;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy, IndexWriter, IndexReader, Searcher, Score};
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use once_cell::sync::OnceCell;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::borrow::{BorrowMut, Borrow};
use serde::Serialize;

mod demo;

use demo::*;
use std::any::Any;

mod snippet;

use snippet::{Snippet, SnippetGenerator};


static SE: OnceCell<Mutex<SearchEngine>> = OnceCell::new();


pub struct SearchEngine {
    path: String,
    schema: String,
    index: Option<Index>,
    index_writer: Option<IndexWriter>,
    index_reader: Option<IndexReader>,
}

impl SearchEngine {
    pub fn new(path: &str, schema: &str) -> Self {
        debug!("new path={}, schema={}", path, schema);
        SearchEngine { path: path.to_string(), schema: schema.to_string(), index: None, index_reader: None, index_writer: None }
    }

    pub fn open(&mut self) -> Result<()> {
        let dir = tantivy::directory::MmapDirectory::open(&self.path).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        let mut index = if Index::exists(&dir)? {
            let index = Index::open_in_dir(&self.path).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
            index
        } else {
            let schema = schema_from_json(&self.schema).map_err(|err| {
                debug!("{}", err.to_string());
                err
            })?;
            let index = Index::create_in_dir(&self.path, schema).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
            index
        };

        let tokenizer = CangJieTokenizer {
            worker: Arc::new(Jieba::empty()), // empty dictionary
            option: TokenizerOption::Unicode,
        };
        index.tokenizers().register(CANG_JIE, tokenizer);

        self.index = Some(index);

        self.init_reader().map_err(|err| {
            debug!("{}", err.to_string());
            err
        })?;
        self.init_writer().map_err(|err| {
            debug!("{}", err.to_string());
            err
        })?;
        Ok(())
    }
    pub fn exists(&mut self) -> Result<bool> {
        let dir = tantivy::directory::MmapDirectory::open(&self.path).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        if Index::exists(&dir).map_err(|err| {
            debug!("{}", err.to_string());
            err
        })? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn init_writer(&mut self) -> Result<()> {
        if self.index_writer.is_none() {
            if self.index.is_none() {
                self.open().map_err(|err| {
                    debug!("{}", err.to_string());
                    err
                })?;
            }
            let writer = self.index.borrow_mut().as_mut().ok_or(Error::msg("index not found"))?.writer(50_000_000).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
            self.index_writer = Some(writer);
        }
        Ok(())
    }

    fn init_reader(&mut self) -> Result<()> {
        if self.index_reader.is_none() {
            if self.index.is_none() {
                self.open().map_err(|err| {
                    debug!("{}", err.to_string());
                    err
                })?;
            }
            let reader = self.index.borrow().as_ref().ok_or(Error::msg("index not found"))?.reader().map_err(|err| { Error::msg(format!("{:?}", err)) })?;
            self.index_reader = Some(reader);
        }
        Ok(())
    }

    pub fn index(&mut self, doc: &str) -> Result<()> {
        let mut writer = self.index_writer.borrow_mut().as_mut().ok_or(Error::msg("writer not found"))?;
        let schema = self.index.borrow().as_ref().ok_or(Error::msg("index not found"))?.schema();
        let doc = schema.parse_document(doc).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        writer.add_document(doc);
        writer.commit().map_err(|err| { Error::msg(format!("{:?}", err)) })?;

        let reader = self.index_reader.borrow_mut().as_ref().ok_or(Error::msg("reader not found"))?;
        reader.reload().map_err(|err| { Error::msg(format!("{:?}", err)) })?;

        Ok(())
    }

    pub fn get_by_term(&mut self, uni_key: &str, value: Value) -> Result<Option<String>> {
        let reader = self.index_reader.borrow_mut().as_ref().ok_or(Error::msg("reader not found"))?;
        let searcher = reader.searcher();

        let schema = self.index.borrow().as_ref().ok_or(Error::msg("index not found"))?.schema();
        let field = if let Some(v) = schema.get_field(uni_key) {
            v
        } else {
            return Ok(None);
        };

        let entry = schema.get_field_entry(field);

        let field_type = entry.field_type();


        let term = match field_type {
            FieldType::Str(_) => Term::from_field_text(field, value.text().unwrap_or_default()),
            FieldType::U64(_) => Term::from_field_u64(field, value.u64_value().unwrap_or_default()),
            FieldType::I64(_) => Term::from_field_i64(field, value.i64_value().unwrap_or_default()),
            FieldType::F64(_) => Term::from_field_f64(field, value.f64_value().unwrap_or_default()),
            FieldType::Date(_) => Term::from_field_date(field, value.date_value().ok_or(Error::msg("invalid date"))?),
            FieldType::HierarchicalFacet => Term::from_facet(field, &Facet::from_text(value.text().unwrap_or_default())),
            // FieldType::Bytes => Term::from_field_bytes(field, ),
            _ => {
                return Err(Error::msg(format!("Value {:?} do not support", value)));
            }
        };

        let term_query = TermQuery::new(term.clone(), IndexRecordOption::Basic);

        let top_docs = searcher.search(&term_query, &TopDocs::with_limit(1)).map_err(|err| { Error::msg(format!("{:?}", err)) })?;


        if let Some((_score, doc_address)) = top_docs.first() {
            let doc = searcher.doc(*doc_address).map_err(|err| { Error::msg(format!("{:?}", err)) })?;

            let mut content = HashMap::new();
            let named_doc = schema.to_named_doc(&doc);
            for (k, v) in named_doc.0 {
                content.insert(k, v[0].clone());
            }
            if content.len() == 0 {
                return Ok(None);
            }
            let s = serde_json::to_string(&content)?;
            Ok(Some(s))
        } else {
            println!("333333");
            Ok(None)
        }
    }

    fn delete(&mut self, uni_key: &str, value: Value) -> Result<()> {
        let schema = self.index.borrow().as_ref().ok_or(Error::msg("index not found"))?.schema();
        if let Some(field) = schema.get_field(uni_key) {
            let entry = schema.get_field_entry(field);
            let field_type = entry.field_type();

            let term = match field_type {
                FieldType::Str(_) => Term::from_field_text(field, value.text().unwrap_or_default()),
                FieldType::U64(_) => Term::from_field_u64(field, value.u64_value().unwrap_or_default()),
                FieldType::I64(_) => Term::from_field_i64(field, value.i64_value().unwrap_or_default()),
                FieldType::F64(_) => Term::from_field_f64(field, value.f64_value().unwrap_or_default()),
                FieldType::Date(_) => Term::from_field_date(field, value.date_value().ok_or(Error::msg("invalid date"))?),
                FieldType::HierarchicalFacet => Term::from_facet(field, &Facet::from_text(value.text().unwrap_or_default())),
                // FieldType::Bytes => Term::from_field_bytes(field, ),
                _ => {
                    return Err(Error::msg(format!("Value {:?} do not support", value)));
                }
            };

            let reader = self.index_reader.borrow_mut().as_ref().ok_or(Error::msg("reader not found"))?;

            let mut writer = self.index_writer.borrow_mut().as_mut().ok_or(Error::msg("writer not found"))?;
            let n = writer.delete_term(term);
            writer.commit().map_err(|err| { Error::msg(format!("{:?}", err)) })?;
            reader.reload().map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        }
        Ok(())
    }


    pub fn search(&mut self, query: &str, fields: Vec<String>, page_start: usize, page_size: usize) -> Result<String> {
        let page_start = if page_start != 0 { page_start - 1 } else { 0 };
        let indexer = self.index.borrow().as_ref().ok_or(Error::msg("index not found"))?;
        let reader = self.index_reader.borrow_mut().as_ref().ok_or(Error::msg("reader not found"))?;
        let searcher = reader.searcher();
        let schema = self.index.borrow().as_ref().ok_or(Error::msg("schema not found"))?.schema();

        let mut search_fields = vec![];
        for v in fields.iter() {
            if let Some(field) = schema.get_field(v.as_str()) {
                search_fields.push(field);
            }
        }
        let query_parser = QueryParser::for_index(indexer, search_fields.clone());
        let query = query_parser.parse_query(query).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        let collector = TopDocs::with_limit(page_size)
            .and_offset(page_start);
        let top_docs = searcher.search(&query, &collector)
            .map_err(|err| { Error::msg(format!("{:?}", err)) })?;


        let mut results = vec![];
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address).map_err(|err| { Error::msg(format!("{:?}", err)) })?;

            // let text_field = retrieved_doc.
            // let mut snippet_generator = SnippetGenerator::create(&searcher, &*query, text_field)?;
            // snippet_generator.set_max_num_chars(100);
            // let snippet = snippet_generator.snippet_from_doc(&doc);

            let mut snippet = "".to_string();

            for v in &search_fields {
                if let Ok(s) = self.snippet(&searcher, &*query, *v, &retrieved_doc) {
                    snippet = s;
                    break;
                }
            }

            let mut content = HashMap::new();
            let named_doc = schema.to_named_doc(&retrieved_doc);
            for (k, v) in named_doc.0 {
                content.insert(k, v[0].clone());
            }


            results.push(SearchResult { snippet, result: content });
        }
        let x = serde_json::to_string(&results)?;
        Ok(x)
    }
    pub fn snippet(&self, searcher: &Searcher, query: &dyn Query, field: Field, doc: &Document) -> Result<String> {
        let mut snippet_generator = SnippetGenerator::create(searcher, query, field).map_err(|err| { Error::msg(format!("{:?}", err)) })?;
        snippet_generator.set_max_num_chars(50);
        let snippet = snippet_generator.snippet_from_doc(doc);
        // println!("{}", snippet.to_mark());
        Ok(snippet.to_mark())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    snippet: String,
    result: HashMap<String, Value>,
}


fn schema_from_json(schema: &str) -> Result<Schema> {
    let h = serde_json::from_str::<HashMap<String, String>>(schema).map_err(|err|{
        error!("schema_from_json schema={},  err: {}", schema, err.to_string());
        err
    })?;
    let mut schema_builder = Schema::builder();
    for (k, v) in h {
        match v.as_str() {
            "string" => { schema_builder.add_text_field(&k, STRING | STORED); }
            "text" => {
                let text_indexing = TextFieldIndexing::default()
                    .set_tokenizer(CANG_JIE) // Set custom tokenizer
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions);
                let text_options = TextOptions::default()
                    .set_indexing_options(text_indexing)
                    .set_stored();
                schema_builder.add_text_field(&k, text_options);
            }
            "u64" => { schema_builder.add_u64_field(&k, INDEXED | STORED); }
            "i64" => { schema_builder.add_i64_field(&k, INDEXED | STORED); }
            "f64" => { schema_builder.add_f64_field(&k, INDEXED | STORED); }
            "date" => { schema_builder.add_date_field(&k, INDEXED | STORED); }
            "facet" => { schema_builder.add_facet_field(&k); }
            "bytes" => { schema_builder.add_bytes_field(&k, INDEXED | STORED); }
            &_ => {
                return Err(Error::msg("字段类型不识别"));
            }
        }
    }
    Ok(schema_builder.build())
}


pub fn open(path: &str, schema: &str) -> Result<()> {
    // let mut key_found = false;
    // let h = serde_json::from_str::<HashMap<String, String>>(schema)?;
    // for (k, v) in h {
    //     if &v == "key" {
    //         key_found = true;
    //     }
    // }
    // if !key_found {
    //     return Err(Error::msg("At least one field must set as 'key' in schema"));
    // }


    let se = SE.get_or_init(|| {
        Mutex::new(SearchEngine::new(path, schema))
    });
    SE.get().unwrap().lock().unwrap().open().map_err(|err|{
        error!("open path={}, schema={}, err: {}", path, schema, err.to_string());
        err
    })?;
    Ok(())
}


pub async fn exists() -> Result<bool> {
    SE.get().unwrap().lock().unwrap().exists()
}

pub async fn index(doc: &str) -> Result<()> {
    SE.get().unwrap().lock().unwrap().index(doc).map_err(|err|{
        error!("index err: {}", err.to_string());
        err
    })?;
    Ok(())
}

// pub async fn search(query: &str, page_start: usize, page_size: usize) -> Result<String> {
//     let results = SE.get().unwrap().lock().unwrap().search(query, &vec![], page_start, page_size)?;
//     Ok(results)
// }

pub async fn search(query: &str, fields: Vec<String>, page_start: usize, page_size: usize) -> Result<String> {
    let results = SE.get().unwrap().lock().unwrap().search(query, fields, page_start, page_size).map_err(|err|{
        error!("search query={},  err: {}", query,  err.to_string());
        err
    })?;
    Ok(results)
}


pub async fn update_by_u64(uni_key: &str, value: u64, new_doc: &str) -> Result<()> {
    let mut se = SE.get().unwrap().lock().unwrap();
    se.delete(uni_key, Value::from(value)).map_err(|err|{
        error!("update_by_u64 key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    se.index(new_doc)?;
    Ok(())
}

pub async fn update_by_str(uni_key: &str, value: &str, new_doc: &str) -> Result<()> {
    let mut se = SE.get().unwrap().lock().unwrap();
    se.delete(uni_key, Value::from(value)).map_err(|err|{
        error!("update_by_str key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    se.index(new_doc)?;
    Ok(())
}

pub async fn delete_by_u64(uni_key: &str, value: u64) -> Result<()> {
    let mut se = SE.get().unwrap().lock().unwrap();
    se.delete(uni_key, Value::from(value)).map_err(|err|{
        error!("delete_by_u64 key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    Ok(())
}

pub async fn delete_by_str(uni_key: &str, value: &str) -> Result<()> {
    let mut se = SE.get().unwrap().lock().unwrap();
    se.delete(uni_key, Value::from(value)).map_err(|err|{
        error!("delete_by_str key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    Ok(())
}


pub async fn get_by_term_u64(uni_key: &str, value: u64) -> Result<Option<String>> {
    let x = SE.get().unwrap().lock().unwrap().get_by_term(uni_key, Value::from(value)).map_err(|err|{
        error!("get_by_term_u64 key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    Ok(x)
}
pub async fn get_by_term_i64(uni_key: &str, value: i64) -> Result<Option<String>> {
    let x = SE.get().unwrap().lock().unwrap().get_by_term(uni_key, Value::from(value)).map_err(|err|{
        error!("get_by_term_i64 key={}, value={}, err: {}", uni_key, value, err.to_string());
        err
    })?;
    Ok(x)
}


// #[test]
// fn test_serde_value() {
//     let x = r#"{"id": "1234567", "title": "2019冠状病毒病疫情", "body":"2019冠状病毒病疫情[9][注 4]，是一次由严重急性呼吸系统综合征冠状病毒2（SARS-CoV-2）所引发的全球大流行疫情[10]。疫情最初在2019年12月于中华人民共和国湖北省武汉市被发现，随后在2020年初迅速扩散至全球多国，逐渐变成一场全球性大瘟疫[11]，被联合国秘书长安东尼欧·古特瑞斯形容为自第二次世界大战以来全球面临的最严峻危机[12][13]。截至2020年12月24日，全球已有191个国家和地区累计报告逾7,870.4万名确诊病例，其中逾173万人因而死亡[1]，是人类历史上致死人数最多的流行病之一。"}"#;
//     let s = serde_json::from_str::<serde_json::Value>(x).unwrap();
//     let mut schema = HashMap::new();
//     schema.insert("id".to_string(), Type::Str);
//     schema.insert("title".to_string(), Type::Str);
//     schema.insert("body".to_string(), Type::Str);
//     let schema = schema_from_map(&schema);
//     let mut se = SearchEngine::new("./db/", schema);
//     se.open();
//
//     // se.index(x).unwrap();
//
//     // se.delete("id", "1234567").unwrap();
//
//     let fields = vec!["title".to_string(), "body".to_string()];
//     let results = se.search(&fields, "病毒", 1, 10).unwrap();
//     println!("{}", results.len());
//     println!("{:?}", results);
// }

#[test]
fn test_str_to_schema() {
    let s = schema_from_json(DEMO_SCHEMA).unwrap();
    let f = s.fields();
    for (k, v) in f {
        println!("{:?}", v);
    }
}

#[tokio::test]
async fn test_open() {
    open("./db", DEMO_SCHEMA).unwrap();

    let data = serde_json::from_str::<Vec<DemoMessage>>(DEMO_DATA).unwrap();

    let now = std::time::Instant::now();

    for v in data {
        let s = serde_json::to_string(&v).unwrap();
        index(&s).await.unwrap();
    }

    println!("{}", now.elapsed().as_millis());
}

#[tokio::test]
async fn test_get_by_term() {
    open("./db", DEMO_SCHEMA).unwrap();

    // delete("id", "333333").unwrap();

    let x = get_by_term_i64("message_id", 141906710246850560).await.unwrap();
    println!("{:?}", x);
}

#[tokio::test]
async fn test_search() {
    open("./db", DEMO_SCHEMA).unwrap();

    // delete("id", "333333").unwrap();

    // let x = search(r#"message_id:141906710246850560 content:聚会"#, 1, 10).unwrap();
    let x = search(r#"message_id:[141906710162964480 TO 141906710192324608}"#, vec!["content".to_string()], 1, 10).await.unwrap();
    println!("{}", x);
}

#[tokio::test]
async fn test_update() {
    let s = r#"{"message_id": "key","user_id": "key", "guild_id": "key", "channel_id": "key", "timestamp": "date", "message": "content"}"#;
    open("./db", s).unwrap();

    let doc = r#"{"id": "1234567", "title": "2019冠状病毒病疫情", "body":"2019冠状病毒病疫情[9][注 4]，是一次由严重急性呼吸系统综合征冠状病毒2（SARS-CoV-2）所引发的全球大流行疫情[10]。疫情最初在2019年12月于中华人民共和国湖北省武汉市被发现，随后在2020年初迅速扩散至全球多国，逐渐变成一场全球性大瘟疫[11]，被联合国秘书长安东尼欧·古特瑞斯形容为自第二次世界大战以来全球面临的最严峻危机[12][13]。截至2020年12月24日，全球已有191个国家和地区累计报告逾7,870.4万名确诊病例，其中逾173万人因而死亡[1]，是人类历史上致死人数最多的流行病之一。"}"#;
    index(doc).await.unwrap();
    let x = search("疫情", vec!["content".to_string()], 1, 10).await.unwrap();
    println!("{:?}", x);

    let doc = r#"{"id": "1234567", "title": "2019冠状病毒病疫情", "body":"疫情最初在2019年12月于中华人民共和国湖北省武汉市被发现，随后在2020年初迅速扩散至全球多国，逐渐变成一场全球性大瘟疫[11]，被联合国秘书长安东尼欧·古特瑞斯形容为自第二次世界大战以来全球面临的最严峻危机[12][13]。截至2020年12月24日，全球已有191个国家和地区累计报告逾7,870.4万名确诊病例，其中逾173万人因而死亡[1]，是人类历史上致死人数最多的流行病之一。"}"#;

    update_by_str("id", "1234567", doc).await.unwrap();
    let x = search("病毒", vec!["content".to_string()], 1, 10).await.unwrap();
    println!("{:?}", x);
}

#[tokio::test]
async fn test_bench() {
    let s = r#"{"message_id": "key","user_id": "key", "guild_id": "key", "channel_id": "key", "timestamp": "date", "message": "content"}"#;
    open("./db", s).unwrap();

    let now = std::time::Instant::now();
    for _ in 0..1 {
        let x = search("冠状", vec!["content".to_string()], 1, 10).await.unwrap();
        println!("{}", x);
    }

    println!("time elapsed: {}", now.elapsed().as_millis());
}

#[tokio::test]
async fn test_delete() {
    let s = r#"{"message_id": "key","user_id": "key", "guild_id": "key", "channel_id": "key", "timestamp": "date", "message": "content"}"#;
    open("./db", s).unwrap();

    delete_by_str("id", "333333");
}