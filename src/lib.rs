#[macro_use]
extern crate lazy_static;

use std::env;
use std::collections::HashMap;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct Config {
    path: String,
    data: Json,
}

fn read_file(path:&str) -> Result<Json, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let json = json!(&s);
    Result::Ok(json)
}

impl Config {

    pub fn new() -> Config {
        let mut map = HashMap::new();
        for argument in env::args() {
            if argument.contains("=") {
                let kv:Vec<&str> = argument.split("=").collect();
                map.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
        if !map.contains_key("target") {
            map.insert(String::from("target"), String::from("dev"));
        }
        // iterate over everything.
        for (key, value) in &map {
            println!("{}: \"{}\"", key, value);
        }

        let target = map.get("target").unwrap();
        let path = format!("./{}/config.json", target);
        let data = read_file(&path).unwrap();
        Config {
            path: String::from("abc"),
            data: data,
        }
    }

    pub fn get_data(&self) -> &Json {
        return &self.data
    }

    pub fn get_path(&self) -> &str {
        return &self.path
    }
}

#[macro_export]
macro_rules! cfg_path {
    ($($y:expr),*) => {{
        let mut data = CFG.get_data();
        $(
            if data.is_array() {
                let array = data.as_array().unwrap();
                let index = usize::from_str($y).unwrap();
                data = array.get(index).unwrap();
            } else {
                data = data.find($y).unwrap();
            }
        )*
        data
    }}
}

#[macro_export]
macro_rules! cfg_str {
    ($($y:expr),*) => {{
        let data = cfg_path!($($y),*);
        data.as_string().unwrap()
    }}
}

#[macro_export]
macro_rules! cfg_i64 {
    ($($y:expr),*) => {{
        let data = cfg_path!($($y),*);
        if data.is_string() {
            let my_str = data.as_string().unwrap();
            i64::from_str(my_str).unwrap()
        } else {
            data.as_i64().unwrap()
        }
    }}
}

lazy_static! {
    pub static ref CFG:Config = Config::new();
}