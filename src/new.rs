use std::path::PathBuf;
use chrono::Local;
use crate::{
    portfolio::Portfolio,
    interaction,
    csv, redirection, currency::Currency
};

pub fn create_portfolio_interactively(portfolio_path: PathBuf) -> Result<(), String> {
    create_portfolio(portfolio_path, interaction::populate_new_portfolio)
}

pub fn create_portfolio_redirected(portfolio_path: PathBuf) -> Result<(), String> {
    create_portfolio(portfolio_path, populate_portfolio_redirected)
}

fn populate_portfolio_redirected(portfolio: &mut Portfolio) {
    let update_table = redirection::collect_portfolio_data();
    let mut categories = vec![];
    let mut data = vec![];

    for (category, value) in update_table.into_iter() {
        categories.push(category);
        data.push(Currency(value));
    }

    portfolio.add_categories(categories);
    portfolio.set_data_for_date(Local::now(), data);
}

fn create_portfolio<F>(portfolio_path: PathBuf, populate: F) -> Result<(), String>
     where F: Fn(&mut Portfolio) 
{
    if portfolio_path.exists() {
        Err(String::from("Portfolio with provided portfolio_path already exists"))
    } else {
        let mut portfolio = Portfolio::new();
        populate(&mut portfolio);
        csv::save_portfolio(&portfolio_path, &portfolio).map_err(|e| format!("Failed to save portfolio: {}", e))?;
        Ok(())
    }
}
