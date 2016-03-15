use std::io;
use std::io::prelude::*;
use std::fs::File;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

fn init() -> Result<Json, io::Error> {
    let mut f = try!(File::open("./dev/config.json"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let json = json!(&s);
    Result::Ok(json)
}

fn main() {
    let s = init();
    println!("{}", s.unwrap());

    //let db = json_path!((&CFG); "db");
    println!("{}", cfg_path!("db"));
    println!("{}", cfg_i64!("db", "conn_limit"));
    println!("{}", cfg_str!("db", "dns"));
}