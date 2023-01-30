/*
this file was inspired from the Rocket hello world exemple
*/

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;

use std::fmt::format;
use rocket::http::Method::{Get, Post};
use rocket::http::Method::{Delete, Put};
use rocket::http::Status;
use rocket::response::{Responder, status};
use rocket::{catcher, Catcher, Data, data, Request, Route, route};
use rocket::data::Outcome;
use rocket::outcome::Outcome::Success;
use rocket::response::status::Custom;
use rocket::tokio::io::AsyncReadExt;
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr::null;
use std::sync::Mutex;
use rocket::form::Form;

#[cfg(test)] mod tests;

// complex constant allocations cannot be executed before main
lazy_static! {
    // the mutex ensures thread-safe access to mutable globals
    static ref store: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[get("/display")]
fn display() -> String {
    let mut result = String::new();
    result.push_str("{\n");
    for (key, val) in store.lock().unwrap().iter() {
        result.push_str(key);
        result.push_str(": ");
        result.push_str(val);
        result.push_str("\n");
    }
    result.push_str("}");
    result
}

#[get("/insert/<key>/<value>")]
fn insert(key: &str, value: &str) -> String {
    let mut result = String::new();

    // copy key and value, for memory safety
    let mut k = String::new();
    k.push_str(key);
    let mut v = String::new();
    v.push_str(value);

    if store.lock().unwrap().contains_key(&mut k) {
        result.push_str("fail");
    } else {
        store.lock().unwrap().insert(k, v);
        result.push_str("ok");
    }

    result
}



#[cfg(test)] mod tests;

#[derive(Debug, FromForm)]
struct InsertForm<'v> {
    key: &'v str,
    value: &'v str,
}

#[post("/", data="<formdata>")]
fn post_insert(formdata: Form<InsertForm>) -> String {
    let mut result = String::new();

    // copy key and value, for memory safety
    let mut k = String::new();
    k.push_str(formdata.key);
    let mut v = String::new();
    v.push_str(formdata.value);

    if store.lock().unwrap().contains_key(&mut k) {
        result.push_str("fail");
    } else {
        store.lock().unwrap().insert(k, v);
        result.push_str("ok");
    }

    result
}





#[get("/delete/<key>")]
fn delete(key: &str) -> String {
    let mut k = String::new();
    k.push_str(key);
    let mut result = String::new();

    if store.lock().unwrap().contains_key(&mut k) {
        store.lock().unwrap().remove(&mut k);
        result.push_str("ok");
    } else {
        result.push_str("fail");
    }
    result
}

#[get("/modify/<key>/<value>")]
fn modify(key: &str, value: &str) -> String {
    let mut result = String::new();
    let mut k = String::new();
    if store.lock().unwrap().contains_key(key) {
        let mut k = String::new();
        k.push_str(key);
        let mut v = String::new();
        v.push_str(value);
        store.lock().unwrap().insert(k, v);

        result.push_str("ok");
    } else {
        result.push_str("fail");
    }
    result
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![display, delete, modify])
        .mount("/", routes![post_insert])
}