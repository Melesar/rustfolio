use std::path::PathBuf;
use chrono::Local;

use crate::currency::Currency;
use crate::redirection;

use super::csv;
use super::portfolio::{self, Portfolio};
use super::interaction;

pub fn add_interactively (file_name: Option<PathBuf>) -> Result<(), String> {
    let (mut portfolio, path) = portfolio::get_portfolio_interactively(file_name)?;
    update_categories(&mut portfolio);
    csv::save_portfolio(&path, &portfolio)?;
    Ok(())
}

pub fn add_redirected(file_name: String) -> Result<(), String> {
    let portfolio_path = portfolio::get_portfolio_path(file_name)?;
    let mut portfolio = csv::read_portfolio(&portfolio_path)?;
    let update_table = redirection::collect_portfolio_data();
    
    let mut data = vec![];
    for category in portfolio.categories() {
        let amount = update_table.get(category)
            .map_or(portfolio.get_latest_value(category).unwrap(), |a| Currency(*a));
        data.push(amount);
    }

    portfolio.set_data_for_date(Local::now(), data);

    csv::save_portfolio(&portfolio_path, &portfolio)
}

pub fn validate_amount(s: &String) -> Result<f32, String> {
    let error_msg = "Amount must be a positive floating point number";
    s.trim()
        .parse::<f32>()
        .map_err(|_| String::from(error_msg))
        .and_then(|f| if f >= 0.0 { Ok(f) } else { Err(String::from(error_msg)) })
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
