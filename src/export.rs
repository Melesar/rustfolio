use std::path::Path;

use super::portfolio;

pub fn export_interactively(portfolio_name: Option<String>, path: &Path) -> Result<(), String> {
    let portfolio = if let Some(name) = portfolio_name {
        portfolio::get_portfolio_contents(name)?
    } else {
        portfolio::get_portfolio_contents_interactively()?
    };

    std::fs::write(path, portfolio).map_err(|e| format!("Failed to export portfolio: {}", e))
}

pub fn export_redirected(portfolio_name: String, path: &Path) -> Result<(), String> {
    let portfolio = portfolio::get_portfolio_contents(portfolio_name)?;
    std::fs::write(path, portfolio).map_err(|e| format!("Failed to export portfolio: {}", e))
}
