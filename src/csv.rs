use std::path::Path;
use chrono::{Local, DateTime, SecondsFormat};
use csv::ErrorKind;

use crate::currency::Currency;

use super::portfolio::Portfolio;

pub fn read_portfolio(path: &Path) -> Result<Portfolio, String> {
    let mut portfolio = Portfolio::new();
    let mut reader = csv::Reader::from_path(path).map_err(|e| {
        match e.kind() {
            ErrorKind::Io(_) => format!("Failed to open file {}. Make sure it exists and you have permission for it", path.to_string_lossy()),
            _ => String::from("Failed to open portfolio file as .csv. Please make sure it's valid")
        }
    })?;

    let headers = reader.headers()
        .map_err(|_| String::from("Failed to read portfolio file. Headers weren't found"))?
        .iter()
        .skip(1);

    for header in headers {
        portfolio.add_category(header.to_string())
    }

    for record in reader.records() {
        let record = record.map_err(|_| String::from("Failed to read portfolio file. Make sure the .csv file is valid"))?;
        let mut iter = record.into_iter();
        let date_string = iter.next().ok_or(String::from("Csv records are expected to have at least one value"))?;

        let date = DateTime::parse_from_rfc3339(date_string)
            .map_err(|_| String::from("Failed to parse record date"))?
            .with_timezone(&Local);

        let mut values = vec![];
        for v in iter {
            let v = v.parse::<f32>().map_err(|_| String::from("Failed to parse record value. Those should be floating-point values"))?;
            values.push(Currency(v));
        }

        portfolio.set_data_for_date(date, values);
    }

    Ok(portfolio)
}

pub fn save_portfolio(path: &Path, portfolio: Portfolio) -> Result<(), String> {
    let mut writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;
    let mut header = vec![""];
    header.extend(portfolio.categories());
    writer.write_record(header).unwrap();

    for value in portfolio.values() {
        let mut record = vec![];
        record.push(value.0.to_rfc3339_opts(SecondsFormat::Secs, false));
        record.extend(value.1.iter().map(|c| c.to_string()));
        writer.write_record(record).unwrap();
    }
    Ok(())
}
