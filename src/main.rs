#![feature(slice_group_by)]

// Modules
mod spreadsheet;

// Imports
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        reqwest::Client::new()
            .get("https://www.mawaqeet.no/download/arendal/?wpdmdl=931")
            .fetch_mode_no_cors()
            .send()
            .await
            .unwrap()
            .bytes()
            .await
    });

    let spreadsheet = future.value().map(|xlsx| match xlsx {
        Ok(xlsx) => Ok(spreadsheet::parse(xlsx)),
        Err(e) => Err(e),
    });
    
    cx.render(match spreadsheet {
        Some(Ok(entries)) => {
            rsx! {
                pre {
                    format!("{entries:#?}")
                }
            }
        }
        Some(Err(_)) => rsx! { div { "Error loading prayer times" } },
        None => rsx! { div { "Loading prayer times..." } },
    })
}
