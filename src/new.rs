use std::path::PathBuf;
use crate::{
    portfolio::Portfolio,
    interaction,
    csv
};

pub fn create_portfolio_interactively(portfolio_path: PathBuf) -> Result<(), String> {
    if portfolio_path.exists() {
        Err(String::from("Portfolio with provided portfolio_path already exists"))
    } else {
        let mut portfolio = Portfolio::new();
        interaction::populate_new_portfolio(&mut portfolio);
        csv::save_portfolio(&portfolio_path, &portfolio).map_err(|e| format!("Failed to save portfolio: {}", e))?;
        Ok(())
    }
}

pub fn create_portfolio_redirected(portfolio_path: PathBuf) -> Result<(), String> {
    Err(String::new())
}
