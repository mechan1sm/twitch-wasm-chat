
use crate::util::*;
use crate::twmsg::*;
use crate::chatui::*;
use crate::twgql::*;


use leptos::*;
use leptos_dom::document;
use web_sys::{ErrorEvent, MessageEvent, WebSocket,HtmlElement};


use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

fn wsnew() -> WebSocket {
  WebSocket::new("wss://irc-ws.chat.twitch.tv:443").unwrap()
}

pub fn ws_create(
  oauth:String,
  nickname:String,
  channel:String,
  gql:Rc<RefCell<gql>>,
  messages_limit:u32
) -> Rc<RefCell<WebSocket>> {
  let ws = wsnew();
  let ws = Rc::new(RefCell::new(ws)); // because wasm is executed in single threads
  //let ws = Arc::new(Mutex::new(ws)); // multi-threaded
  ws.borrow().set_binary_type(web_sys::BinaryType::Arraybuffer);

  let cloned_ws = ws.clone();
  let callback_onopen = Closure::<dyn FnMut()>::new(move || {
    console_log!("socket opened");
    let borrowed_ws = cloned_ws.borrow();
    borrowed_ws.send_with_str(format!("PASS oauth:{}\r\n",oauth).as_str()).unwrap();
    borrowed_ws.send_with_str(format!("NICK {}\r\n",nickname).as_str()).unwrap();
    //borrowed_ws.send_with_str(format!("USER {} 8 * :{}\r\n",nickname,nickname).as_str()).unwrap();
    //borrowed_ws.send_with_str("CAP REQ :twitch.tv/commands\r\n").unwrap();
    //borrowed_ws.send_with_str("CAP REQ :twitch.tv/membership\r\n").unwrap();
    borrowed_ws.send_with_str("CAP REQ :twitch.tv/tags\r\n").unwrap();
    borrowed_ws.send_with_str(format!("JOIN #{}\r\n",channel).as_str()).unwrap();
    drop(borrowed_ws);
    });
  let cloned_ws = ws.clone();
  let callback_onclose = Closure::<dyn FnMut()>::new(move || {
    console_log!("socket closed, trying to reopen...");
    let new_ws = wsnew();
    let old_ws = cloned_ws.replace(new_ws);
    let borrowed_ws = cloned_ws.borrow();
    borrowed_ws.set_onerror(old_ws.onerror().as_ref());
    borrowed_ws.set_onmessage(old_ws.onmessage().as_ref());
    borrowed_ws.set_onclose(old_ws.onclose().as_ref());
    borrowed_ws.set_onopen(old_ws.onopen().as_ref());
    drop(borrowed_ws);
    });

  let callback_onerror = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
      console_log!("error event: {:?}", e);
  });
  
  let element = document().get_element_by_id("chat-area").unwrap();
  let element:HtmlElement = element.unchecked_into();
  let cloned_ws = ws.clone();
  let children = element.children();
  let callback_onmessage = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
      let message = txt.as_string().unwrap();
      for line in message.lines() {
        let mut msg = parsemsg(line);
        if msg.command == "PING" {
          msg.command = "PONG";
          let message = makemsg(&msg);
          //console_log!("WRITE|{:?}",&message);
          cloned_ws.borrow().send_with_str(message.as_str()).unwrap();
        } else if msg.command == "PRIVMSG" {
          let v = ui_msgview(&msg,&gql.borrow());
          mount_to(element.clone(), move || v);
          while children.length() > messages_limit {
            match children.get_with_index(0) {
              Some(el) => {el.remove();}
              None => ()
            }
          }
        } else if (msg.command == "JOIN") || (msg.command == "PART") {
          //let sl = String::from(line);
          //mount_to(element.clone(),move || view! { <p>{sl}</p> });
        } else {
          //console_log!("{}", line);
        }
      }
      element.set_scroll_top(element.scroll_height());

    } else {
        console_log!("message event, received Unknown: {:?}", e.data());
    }
  });
  
  let borrowed_ws = ws.borrow();
  borrowed_ws.set_onerror(Some(callback_onerror.as_ref().unchecked_ref()));
  callback_onerror.forget();

  borrowed_ws.set_onmessage(Some(callback_onmessage.as_ref().unchecked_ref()));
  callback_onmessage.forget();
  
  borrowed_ws.set_onclose(Some(callback_onclose.as_ref().unchecked_ref()));
  callback_onclose.forget();
  
  borrowed_ws.set_onopen(Some(callback_onopen.as_ref().unchecked_ref()));
  callback_onopen.forget();
  drop(borrowed_ws);
  ws
}


