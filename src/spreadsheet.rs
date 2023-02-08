use anyhow::Result;
use std::io::Cursor;
use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

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
    fajr: f64,
    shurooq: f64,
    dhuhr: f64,
    asr_shafi: f64,
    asr_hanafi: f64,
    maghrib: f64,
    isha: f64,
}

pub fn parse(spreadsheet: &[u8]) -> Result<Vec<Entry>> {
    // Get spreadsheet
    let reader = Cursor::new(spreadsheet);
    let mut workbook = Xlsx::new(reader)?;
    
    let mut workbook: Xlsx<_> = open_workbook("arendal.xlsx")?;
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

    // Group each row delimited by empty row, then map them into Month with name and entries
    let months = rows
        .group_by(|first, second| first.0.len() > 0 && second.0.len() > 0)
        .into_iter()
        .filter(|group| group.len() > 1)
        .map(|group| {
            group[2..].into_iter().flat_map(|row| {
                let entry = Entry {
                    fajr: row.2.parse().ok()?,
                    shurooq: row.3.parse().ok()?,
                    dhuhr: row.4.parse().ok()?,
                    asr_shafi: row.5.parse().ok()?,
                    asr_hanafi: row.6.parse().ok()?,
                    maghrib: row.7.parse().ok()?,
                    isha: row.8.parse().ok()?,
                };
                Some(entry)
            }).collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // Return months
    Ok(months)
}
