use dioxus::prelude::*;

use crate::{IngestInput, PageTitle};

pub fn IngestPage(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "container mx-auto px-4 py-4",
        PageTitle {
          title: "Ingest".to_string(),
          subtitle: "Add your videos to the vector store".to_string()
        },
        IngestInput {}

      }
    })
}
