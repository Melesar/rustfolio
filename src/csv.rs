use std::path::PathBuf;
use super::portfolio::Portfolio;

pub fn read_portfolio(path: &PathBuf) -> Result<Portfolio, String> {
    let mut portfolio = Portfolio::new();
    let mut reader = csv::Reader::from_path(path).map_err(|e| e.to_string())?;

    let headers = reader.headers()
        .map_err(|_| String::from("Failed to read portfolio file. Headers weren't found"))?
        .clone();
    let values = reader.records()
        .last()
        .ok_or_else(|| String::from("Failed to read portfolio file. Records weren't found"))?
        .map_err(|e| format!("Failed to read portfolio file: {}", e))?
        .into_iter()
        .map(|v| v.parse::<f32>().map_err(|_| String::from("Failed to read portfolio file. Invalid data format")))
        .collect::<Vec<Result<f32, String>>>();

    for (header, value) in headers.into_iter().zip(values.into_iter()) {
        portfolio.add_category(header.to_string(), value?);
    }

    Ok(portfolio)
}

pub fn save_portfolio(path: &PathBuf, portfolio: Portfolio) -> Result<(), String> {
    let mut writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;
    writer.write_record(portfolio.categories()).unwrap();
    writer.write_record(portfolio.values().map(|c| c.to_string())).unwrap();
    Ok(())
}
