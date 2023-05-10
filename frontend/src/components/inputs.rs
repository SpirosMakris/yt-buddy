use dioxus::prelude::*;

pub fn IngestInput(cx: Scope) -> Element {
    let yt_url = use_state(cx, || "".to_string());

    cx.render(rsx! {
      div {
        class: "flex flex-col items-start justify-start my-4",
        input {
          class: "w-full px-3 py-2 text-gray-700 border rounded-lg focus:outline-none",
          placeholder: "YouTube URL",
          autofocus: "true",
          value: "{yt_url}",
          oninput: move |e| {
            yt_url.set(e.value.clone());
          }
        },
        div {
          "{yt_url}"
        },
        button {
          class: "px-4 py-2 mt-2 text-white bg-indigo-500 rounded-lg hover:bg-indigo-400",
          "Ingest"
        }
      }
    })
}
