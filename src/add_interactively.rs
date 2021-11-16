use std::path::PathBuf;

use super::csv;
use super::portfolio::{self, Portfolio};
use super::interaction;

pub fn add (file_name: Option<PathBuf>) -> Result<(), String> {
    let (portfolio, path) = portfolio::get_portfolio_interactively(file_name)?;
    if let Some(mut p) = portfolio {
        update_categories(&mut p);
        csv::save_portfolio(&path, p)?;
    } else {
        create_new_portfolio(path);
    }

    Ok(())
}

fn create_new_portfolio(path: PathBuf) {
    let portfolio_name = path.file_stem().unwrap();
    let confirmation_label = format!("Portfolio {} doesn't exist yet. Create?", portfolio_name.to_string_lossy());
    if !interaction::confirmation(&confirmation_label, true) { 
        return;
    }

    let mut portfolio = Portfolio::new();
    interaction::populate_new_portfolio(&mut portfolio);
    if let Err(s) = csv::save_portfolio(&path, portfolio) {
        eprintln!("{}", s);
    }
}

fn update_categories(portfolio: &mut Portfolio) {
    for (category, current_amount) in portfolio.data_mut() {
        **current_amount = interaction::ask_for_input(&format!("Amount for {}", category), validate_amount);
    }
}


pub fn validate_amount(s: &String) -> Result<f32, String> {
    let error_msg = "Amount must be a positive floating point number";
    s.trim()
        .parse::<f32>()
        .map_err(|_| String::from(error_msg))
        .and_then(|f| if f >= 0.0 { Ok(f) } else { Err(String::from(error_msg)) })
}
