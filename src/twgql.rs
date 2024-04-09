use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, Headers,ReferrerPolicy};

use serde_wasm_bindgen;
use serde_json::{self,json};
use leptos::*;

use std::collections::HashMap;


pub struct badge {
  pub srcset: String,
  pub alt: String
}

pub struct gql {
  pub badges: HashMap<String,badge>,
  id:String
}
impl gql {
  pub fn new() -> gql {
    gql {
      badges:HashMap::new(),
      id: String::default()
    }
  }
}
struct query;
impl query {
  fn badges(channel:&str) -> serde_json::Value {
    json!({
      "operationName": "ChatList_Badges",
      "variables": { 
        "channelLogin": channel,
      },
      "extensions": {
        "persistedQuery": {
          "version": 1,
          "sha256Hash": "86f43113c04606e6476e39dcd432dee47c994d77a83e54b732e11d4935f0cd08"
        }
      }
    })
  }
  fn uselive(channel:&str) -> serde_json::Value {
    json!({
      "operationName": "UseLive",
      "variables": {
        "channelLogin": channel
      },
      "extensions": {
        "persistedQuery": {
          "version": 1,
          "sha256Hash": "639d5f11bfb8bf3053b424d9ef650d04c4ebb7d94711d644afb08fe9a0fad5d9"
        }
      }
    })
  }
}

struct parse;
impl parse {
  fn badges(mut json:serde_json::Value) -> HashMap<String,badge> {
    let global_badges = json["data"]["badges"].take();
    let channel_badges = json["data"]["user"]["broadcastBadges"].take();
    
    let mut map = HashMap::new();
    for badges_it in [global_badges,channel_badges] {
      if let serde_json::Value::Array(badges) = badges_it {
        for mut badge in badges {

          let key = format!("{}/{}",
            badge["setID"].take().as_str().unwrap(),
            badge["version"].take().as_str().unwrap()
          );
          let alt = match badge["title"].take() {serde_json::Value::String(st) => st, _ => unreachable!()};
          let mut srcs = Vec::with_capacity(3);
          for (i,x) in [("image1x","1x"),("image2x","2x"),("image4x","4x")] {
            if let serde_json::Value::String(mut st) = badge[i].take() {
              st = format!("{} {}",st,x);
              srcs.push(st);
            }
          }
          map.insert(key,badge {
            srcset: srcs.join(", "),
            alt
          });
        }
      }
    }
    map
  }
  fn uselive(mut json:serde_json::Value) -> String {
    match json["data"]["user"]["id"].take() {serde_json::Value::String(st) => st, _ => unreachable!()}
  }
}

pub fn gql_request(body:&JsValue) -> JsFuture {
  let headers = Headers::new().unwrap();
  headers.append("Client-Id","kimne78kx3ncx6brgo4mv6wki5h1ko").unwrap();
  headers.append("Client-Version","1f156351-2ff1-4b5d-8309-64564c5f83f5").unwrap();
  let mut request_init = RequestInit::new();
  request_init
    .method("POST")
    .referrer_policy(ReferrerPolicy::StrictOriginWhenCrossOrigin)
    .referrer("https://www.twitch.tv/")
    .headers(&headers)
    .body(Some(body));
  let request = Request::new_with_str_and_init(
    "https://gql.twitch.tv/gql#origin=twilight",
    &request_init
  ).unwrap();
  let response_future = JsFuture::from(window().fetch_with_request(&request));
  response_future
}

pub async fn perform_gql(channel:&str) -> gql {
  let body = JsValue::from_str(
    serde_json::Value::from(vec![
      query::badges(channel),
      query::uselive(channel)
  ]).to_string().as_str());

  let resp_value_fut = gql_request(&body);
  let resp_value = resp_value_fut.await.unwrap();
  let response:Response = resp_value.dyn_into().unwrap();
  let jsval = JsFuture::from(response.json().unwrap()).await.unwrap();
  let mut json_arr:serde_json::Value = serde_wasm_bindgen::from_value(jsval).unwrap();
  //assert_eq!(serde_json::Value::Array,json_arr);
  let mut gql_badges:Option<HashMap<String,badge>> = None;
  let mut gql_id:Option<String> = None;
   //serde_json::Value::Array(arr) = json_arr {
  let arr = json_arr.as_array_mut().unwrap();
  for j in arr.iter_mut() {
    //console_log!("{:?}",j["extensions"]["operationName"].as_str().unwrap());
    match j["extensions"]["operationName"].as_str().unwrap() {
      "UseLive" => { gql_id = Some(parse::uselive(j.take())); },
      "ChatList_Badges" => { gql_badges = Some(parse::badges(j.take())); },
      &_ => { unreachable!("unknown operationName"); }
    }
  }
  gql {
    badges:gql_badges.unwrap(),
    id:gql_id.unwrap()
  }
}

