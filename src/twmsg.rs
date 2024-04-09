
use regex::{Regex,Captures};
use lazy_static::lazy_static;

lazy_static! {
  static ref MSG_RE:Regex = Regex::new(r"(?:@(\S+) )?(?::(\S+) )?(\S+)(?: (.+))?").unwrap();
  static ref TAGS_RE:Regex = Regex::new(r"([^=]+)=([^;]*);?").unwrap();
  static ref PARAMS_RE:Regex = Regex::new(r"([^:]\S* |(?::).+)").unwrap();
}

pub struct twmsg<'a> {
  pub tags:Option<&'a str>,
  pub prefix:Option<&'a str>,
  pub command:&'a str,
  pub params:Option<&'a str>,
}

#[derive(Default)]
pub struct twtags<'a> {
  pub badges:&'a str,
  pub color:&'a str,
  pub display_name:&'a str,
  //pub mod:bool,
  //pub vip:bool
}

pub fn parsemsg(message:&str) -> twmsg {
  let captures:Captures = MSG_RE.captures(&message).unwrap();
  twmsg {
    command:captures.get(3).map(|m| m.as_str()).unwrap(),
    tags:   captures.get(1).map(|m| m.as_str()),
    prefix: captures.get(2).map(|m| m.as_str()),
    params: captures.get(4).map(|m| m.as_str()),
  }
}

pub fn parsetags(msgtags:&str) -> twtags {
  let mut tags = twtags::default();
  //console_log!("tags|{}",msgtags);
  //let captures:Captures = TAGS_RE.captures(tags).unwrap();
  for cap in TAGS_RE.captures_iter(&msgtags) {
      match &cap[1] {
        "badges" => {tags.badges = cap.get(2).unwrap().as_str()},
        "color" => {tags.color = cap.get(2).unwrap().as_str()},
        "display-name" => {tags.display_name = cap.get(2).unwrap().as_str()}
        _ => ()
      }
    }
  tags
}
pub fn parseparams(params:&str) -> &str {
  let mut last:&str = "";
  for cap in PARAMS_RE.captures_iter(params) {
    last = cap.get(1).unwrap().as_str();
  }
  &last[1..]
}

pub fn makemsg(msg:&twmsg) -> String {
  format!("{}{}{}{}\r\n",
    match msg.tags {
      Some(ref st) => format!("@{} ",st),
      None => String::default()
    },
    match msg.prefix {
      Some(ref st) => format!(":{} ",st),
      None => String::default()
    },
    msg.command,
    match msg.params {
      Some(ref st) => format!(" {}",st),
      None => String::default()
    }
  )

}

