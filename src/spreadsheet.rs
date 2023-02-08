use anyhow::Result;
use std::io::Cursor;
use calamine::{Error, RangeDeserializerBuilder, Reader, Xlsx};

type Row = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

#[derive(Debug)]
pub struct Entry {
    day: usize,
    fajr: f64,
    shurooq: f64,
    dhuhr: f64,
    asr_shafi: f64,
    asr_hanafi: f64,
    maghrib: f64,
    isha: Option<f64>,
}

pub fn parse(spreadsheet: &[u8]) -> Result<Vec<Entry>> {
    // Get spreadsheet
    let reader = Cursor::new(spreadsheet);
    let mut workbook = Xlsx::new(reader)?;
    let range = workbook
        .worksheet_range("Salah tabell")
        .ok_or_else(|| Error::Msg("Cannot find 'Sheet1'"))??;

    // Map each row into tuple
    let rows = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .flat_map(|row| {
            let items = row?;
            Ok::<Row, calamine::Error>(items)
        })
        .collect::<Vec<_>>();

    // Group each row delimited by empty row
    let groups = rows
        .group_by(|first, second| first.0.len() > 0 && second.0.len() > 0)
        .into_iter()
        .filter(|group| group.len() > 1);

    // Map each row into entry and add to entries
    let mut entries = Vec::with_capacity(365);
    for (i, group) in groups.into_iter().enumerate() {
        let slice = if i == 0 {
            &group[1..]
        } else {
            &group[2..]
        };
        
        for row in slice {
            let entry = Entry {
                day: row.0.parse()?,
                fajr: row.2.parse()?,
                shurooq: row.3.parse()?,
                dhuhr: row.4.parse()?,
                asr_shafi: row.5.parse()?,
                asr_hanafi: row.6.parse()?,
                maghrib: row.7.parse()?,
                isha: row.8.parse().ok(),
            };
            entries.push(entry);
        }
    }
    
    // Return entries
    Ok(entries)
}
