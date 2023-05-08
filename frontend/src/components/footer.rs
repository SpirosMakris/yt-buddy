use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
          class: "fixed bottom-0 left-0 right-0 z-40 px-4 py-3 text-center text-white bg-gray-800",
          "This is a footer"
        }

    })
}
