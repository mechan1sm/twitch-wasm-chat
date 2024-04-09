mod util;

mod chatws;
use chatws::*;

mod twgql;
use twgql::*;
mod twmsg;

mod chatui;

use leptos::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, Response, Headers, ReferrerPolicy};

use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;

//use crate::util::*;

#[wasm_bindgen]
pub fn app() {
  
  let url = window().location();
  let search = url.search().unwrap();
  let mut messages_limit = 100;
  let mut channel:String = String::default();
  let mut rng = rand::thread_rng();
  let random_number:String = format!("{}",rng.gen_range(1000..=80999));
  if (search.len() > 0) && (&search[..1] == "?") {
    for param in search[1..].split('&') {
      let mut it = param.split('=');
      let key = it.next().unwrap();
      let value = it.next();
      if value == None {
        continue;
      }
      match key {
        "channel" => {channel = String::from(value.unwrap());},
        "limit" => {messages_limit = value.unwrap().parse().unwrap();},
        &_ => ()
      }
    }
  }
  let channel2 = channel.clone();
  let gql:Rc<RefCell<gql>> = Rc::new(RefCell::new(gql::new()));
  let cloned_gql = gql.clone();


  spawn_local(async move {
    let gql_new = perform_gql(&channel).await;
    let gql_old = cloned_gql.replace(gql_new);
    drop(gql_old);
  });

  let cloned_gql = gql.clone();
  spawn_local(async move {
    let ws = ws_create(
      // "justinfan".concat(Math.floor(8e4 * Math.random() + 1e3),);
      // "justinfan" + [1000,80999]
      "SCHMOOPIIE".to_string(),
      format!("justinfan{}",random_number),
      channel2,
      cloned_gql,
      messages_limit
    ); 
  });
}


