
use crate::twmsg::*;
use crate::twgql::*;

use leptos::*;


pub fn ui_msgview(msg:&twmsg,gql:&gql) -> HtmlElement<html::Div> {
  let tags = parsetags(msg.tags.unwrap());
  let text = parseparams(msg.params.unwrap());
  view! {
  <div class="message">
    {
      tags.badges.split(',').filter_map(|name| match gql.badges.get(name) {
      None => None,
      Some(ref gql) => Some(view! { 
        <img
          class="badge" 
          srcset={gql.srcset.clone()}
          alt={gql.alt.clone()}
        />
      })
    }).collect_view()
    }
    <span
      class="message-username" 
      style={format!("color:{}",tags.color)}
    >{String::from(tags.display_name)}</span>
    {": "}
    <span class="message-text" >{String::from(text)}</span>
  </div>
  }
}

