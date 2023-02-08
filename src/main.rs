#![feature(slice_group_by)]

// Modules
mod spreadsheet;

// Imports
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let bytes = include_bytes!("arendal.xlsx");
    let spreadsheet = spreadsheet::parse(bytes);
    
    // let future = use_future(cx, (), |_| async move {
    //     reqwest::Client::new()
    //         .get("https://www.mawaqeet.no/download/arendal/?wpdmdl=931")
    //         .fetch_mode_no_cors()
    //         .send()
    //         .await
    //         .unwrap()
    //         .bytes()
    //         .await
    // });

    // let spreadsheet = future.value().map(|xlsx| match xlsx {
    //     Ok(xlsx) => Ok(spreadsheet::parse(xlsx)),
    //     Err(e) => Err(e),
    // });
    
    cx.render(match spreadsheet {
        Ok(entries) => {
            rsx! {
                h1 { "Mawaqeet" }
                p { format!("{}", entries.len()) }
                pre {
                    format!("{entries:#?}")
                }
            }
        }
        Err(e) => rsx! { div { "Error while parsing spreadsheet: {e:?}" } },
    })
}
