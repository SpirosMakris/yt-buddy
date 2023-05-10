#![allow(non_snake_case)]

use dioxus::prelude::*;

pub use components::*;
use dioxus_router::{Route, Router};

pub mod components {
    pub mod footer;
    pub mod ingest;
    pub mod inputs;
    pub mod page_title;
    pub mod search;
    pub mod sidenav;

    pub use footer::*;
    pub use ingest::*;
    pub use inputs::*;
    pub use page_title::*;
    pub use search::*;
    pub use sidenav::*;
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
      link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
      Router {
        SideNav {},
        Route { to: "/", IngestPage {} },
        Route { to: "/index.html", SearchPage {} },
        Route { to: "/ingest", IngestPage {} },
        Route { to: "/search", SearchPage {} }
      },
      Footer {}
    })
}
