#![feature(slice_group_by)]

// Modules
mod spreadsheet;

// Imports
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // Get spreadsheet entries
    let bytes = include_bytes!("arendal.xlsx");
    let spreadsheet = spreadsheet::parse(bytes);

    // Get current day in year
    let now = js_sys::Date::new_0();
    let start = js_sys::Date::new_with_year_month_day(now.get_full_year(), 0, 0);
    let day = ((now.value_of() - start.value_of()) / 1000.0 / 60.0 / 60.0 / 24.0).floor() as usize - 1;
    
    cx.render(match spreadsheet {
        Ok(entries) => {
            rsx! {
                h1 { "Mawaqeet" }
                pre {
                    format!("{:#?}", entries[day])
                }
            }
        }
        Err(e) => rsx! { div { "Error while parsing spreadsheet: {e:?}" } },
    })
}
