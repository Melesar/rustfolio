use std::path::PathBuf;
use chrono::Local;

use crate::currency::Currency;

use super::csv;
use super::portfolio::{self, Portfolio};
use super::interaction;

pub fn add (file_name: Option<PathBuf>) -> Result<(), String> {
    let (mut portfolio, path) = portfolio::get_portfolio_interactively(file_name)?;
    update_categories(&mut portfolio);
    csv::save_portfolio(&path, &portfolio)?;
    Ok(())
}

fn update_categories(portfolio: &mut Portfolio) {
    let date = Local::now();
    let data = portfolio.categories()
        .map(|category| {
            let default_value = portfolio.get_latest_value(category);
            let input = interaction::Input::new(format!("Amount for {}", category), validate_amount).default_value(default_value.map(|c| c.0));
            input.ask_for_input().unwrap()
        })
        .map(|amount| Currency(amount))
        .collect::<Vec<Currency>>();

    portfolio.set_data_for_date(date, data);
}


pub fn validate_amount(s: &String) -> Result<f32, String> {
    let error_msg = "Amount must be a positive floating point number";
    s.trim()
        .parse::<f32>()
        .map_err(|_| String::from(error_msg))
        .and_then(|f| if f >= 0.0 { Ok(f) } else { Err(String::from(error_msg)) })
}
