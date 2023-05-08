use dioxus::prelude::*;
use dioxus_router::Link;

pub fn SideNav(cx: Scope) -> Element {
    cx.render(rsx! {
        // Sidebar
        div {
          class: "fixed shadow top-0 left-0 flex flex-shrink-0 transition-all",
          nav {
            class: "z-20 flex-col items-center flex-shrink-0 hidden w-16 py-4 bg-white border-r-2 border-indigo-100 shadow-md sm:flex rounded-tr-3xl rounded-br-3xl",
            // Logo
            Logo {},
            // Nav Items
            div {
              class: "flex flex-col items-center flex-1 p-2 space-y-4",                            
              MenuButton {
                button_type: MenuButtonType::Ingest,
              }, 
              MenuButton {
                button_type: MenuButtonType::Search,
              },                           
            },
            UserAvatar {},
          }          
        }
    })
}

#[derive(PartialEq)]
enum MenuButtonType {
  Ingest,
  Search,
}

#[inline_props]
fn MenuButton(cx: Scope, button_type: MenuButtonType) -> Element {
  cx.render(rsx! {
    button {
      class: "p-2 transition-colors rounded-lg shadow-md hover:bg-indigo-800 hover:text-white focus:outline-none focus:ring focus:ring-indigo-600 focus:ring-offset-white focus:ring-offset-2 text-gray-500 bg-white",
      match button_type {
        MenuButtonType::Ingest => {
          rsx! { 
            Link { to: "/ingest", IconInput {} }
          }
        },
        MenuButtonType::Search => {
          rsx! {
            Link { to: "/search", IconSearch {} }
          }
        }
      }
    }
  })
}

fn Logo(cx: Scope) -> Element {
  cx.render(rsx! {
    div {
      class: "flex-shrink-0 py-4",
      a {
        href: "#",
        IconVideo {
          
        },
      }
    },
  })
}

fn UserAvatar(cx: Scope) -> Element {
  cx.render(rsx! {
    div {
      class: "relative flex items-center flex-shrink-0 p-2",
      "Avatar"
    }
  })
}

fn IconInput(cx: Scope) -> Element {
  cx.render(rsx! {    
      svg {        
        class: "w-8 h-8 fill-gray-400",        
        xmlns: "http://www.w3.org/2000/svg",
        view_box: "0 0 128 128",
        g {
          path {          
            d: "M43.7578,61.7578a5.9994,5.9994,0,1,0,8.4844,8.4844l18-18a5.9979,5.9979,0,0,0,0-8.4844l-18-18a5.9994,5.9994,0,0,0-8.4844,8.4844L51.5156,42H6A6,6,0,0,0,6,54H51.5156Z"
          },
          path {
            d: "M90,0H30a5.9966,5.9966,0,0,0-6,6V18a6,6,0,0,0,12,0V12H84V84H36V78a6,6,0,0,0-12,0V90a5.9966,5.9966,0,0,0,6,6H90a5.9966,5.9966,0,0,0,6-6V6A5.9966,5.9966,0,0,0,90,0Z",
          } 
        }
      }    
  })
}

fn IconSearch(cx: Scope) -> Element {
  cx.render(rsx!{
    svg {
      class: "w-8 h-8 fill-gray-400",
      view_box: "0 0 48 48",
      xmlns: "http://www.w3.org/2000/svg",
      path {
        d: "M31 28h-1.59l-.55-.55C30.82 25.18 32 22.23 32 19c0-7.18-5.82-13-13-13S6 11.82 6 19s5.82 13 13 13c3.23 0 6.18-1.18 8.45-3.13l.55.55V31l10 9.98L40.98 38 31 28zm-12 0a9 9 0 1 1 .001-18.001A9 9 0 0 1 19 28z"
      },
      path {
        d: "M0 0h48v48H0z",
        fill: "none"
      }  
    }
  })
}

fn IconVideo(cx: Scope) -> Element {
  cx.render(rsx! {
    svg {
      class: "w-12 h-auto text-red stroke-cyan-500 hover:stroke-cyan-700",
      view_box: "0 0 32 32",      
      xmlns: "http://www.w3.org/2000/svg",
      path {
        d: "M29.977 7.119C29.981 7.078 30 7.042 30 7V3a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4c0 .042.019.078.024.119.008.068.018.133.041.2s.055.124.09.183c.02.035.027.075.053.108L12 20.34V29a1 1 0 0 0 1.555.832l6-4c.278-.186.445-.498.445-.832v-4.659L29.793 7.61c.025-.034.033-.074.054-.109a.99.99 0 0 0 .089-.182c.022-.067.032-.132.041-.2zM18 24.465l-4 2.667V20c0-.042-.019-.078-.024-.119-.008-.068-.018-.133-.041-.2s-.055-.124-.09-.183c-.021-.035-.028-.075-.053-.108L5.031 8H21a1 1 0 1 0 0-2H4V4h24v2h-4a1 1 0 1 0 0 2h2.969l-8.762 11.39c-.025.034-.033.074-.054.11-.035.059-.066.116-.089.182s-.032.131-.041.2c-.004.04-.023.076-.023.118v4.465z"
      }
    }
  })
}
