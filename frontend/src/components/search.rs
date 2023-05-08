use dioxus::prelude::*;

use crate::PageTitle;

pub fn SearchPage(cx: Scope) -> Element {
    cx.render(rsx! {
      PageTitle {
        title: "Search".to_string(),
        subtitle: "Search and chat with your videos here".to_string()
      }
    })
}
