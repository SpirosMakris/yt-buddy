use dioxus::prelude::*;

use crate::PageTitle;

pub fn IngestPage(cx: Scope) -> Element {
    cx.render(rsx! {
      PageTitle {
        title: "Ingest".to_string(),
        subtitle: "Add your videos to the vector store".to_string()
      }
    })
}
