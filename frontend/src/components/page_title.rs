use dioxus::prelude::*;

#[inline_props]
pub fn PageTitle(cx: Scope, title: String, subtitle: String) -> Element {
    cx.render(rsx! {
      div {
        class: "flex flex-col items-center justify-start h-screen",
        div {
          class: "flex flex-col items-center justify-center",
          h1 {
            class: "text-6xl font-bold text-gray-800",
            "{title}"
          },
          p {
            class: "text-gray-500",
            "{subtitle}"
          }
        }
      }
    })
}
