#![feature(slice_group_by)]

// Modules
mod spreadsheet;

// Imports
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn float_to_time(input: f64) -> String {
    let minutes = (input * 24.0 * 60.0) as usize;
    let hour = minutes / 60;
    let minute = minutes % 60;
    format!("{hour}:{minute:02}")
}

fn app(cx: Scope) -> Element {
    // Get spreadsheet entries
    let bytes = include_bytes!("arendal.xlsx");
    let spreadsheet = spreadsheet::parse(bytes);

    // Get current day in year and entry
    let now = js_sys::Date::new_0();
    let start = js_sys::Date::new_with_year_month_day(now.get_full_year(), 0, 0);
    let day = ((now.value_of() - start.value_of()) / 1000.0 / 60.0 / 60.0 / 24.0).floor() as usize - 1;
    
    cx.render(match spreadsheet {
        Ok(entries) => {
            // Get prayer times
            let entry = &entries[day];
            let fajr = float_to_time(entry.fajr);
            let shuroq = float_to_time(entry.shuroq);
            let dhuhr = float_to_time(entry.dhuhr);
            let asr_shafi = float_to_time(entry.asr_shafi);
            let asr_hanafi = float_to_time(entry.asr_hanafi);
            let maghrib = float_to_time(entry.maghrib);
            let isha = entry.isha.map(|it| float_to_time(it)).unwrap_or(String::from("--::--"));
            
            rsx! {
                h1 { "Mawaqeet" }
                p { format!("Fajr: {fajr}") }
                p { format!("Soloppgang: {shuroq}") }
                p { format!("Dhuhr: {dhuhr}") }
                p { format!("Asr Shafi: {asr_shafi}") }
                p { format!("Asr Hanafi: {asr_hanafi}") }
                p { format!("Maghrib: {maghrib}") }
                p { format!("Isha: {isha}") }
            }
        }
        Err(e) => rsx! { div { "Error while parsing spreadsheet: {e:?}" } },
    })
}
