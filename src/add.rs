use std::collections::HashMap;
use std::path::PathBuf;
use chrono::Local;

use crate::currency::Currency;

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

    let stdin = std::io::stdin();
    let mut buffer = String::new();

    let mut current_category : Option<String> = None;
    let mut update_table = HashMap::new();
    while stdin.read_line(&mut buffer).unwrap_or(0) > 0 {
        if let Some(category) = current_category.as_ref() {
            let amount = validate_amount(&buffer.trim().to_string());

            if amount.is_err() { 
                current_category = None;
                buffer.clear();
                continue;
            }

            update_table.insert(category.clone(), amount.unwrap());
            current_category = None;
        } else {
            current_category = Some(buffer.trim().to_string());
        }

        buffer.clear();
    }

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


