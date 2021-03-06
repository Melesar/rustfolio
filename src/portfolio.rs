use std::path::PathBuf;
use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use super::{csv, interaction};
use super::currency::Currency;
use super::files;

pub struct Portfolio {
    categories: Vec<String>,
    data: BTreeMap<DateTime<Local>, Vec<Currency>>,
}

impl<'a> Portfolio {
    pub fn new() -> Self {
        Portfolio { categories: vec![], data: BTreeMap::default() }
    }

    pub fn data(&'a self) -> Option<impl Iterator<Item=(&'a String, &'a Currency)>> {
        let categories_iter = self.categories.iter();
        let data_entry = self.data.iter().rev().next();

        data_entry.and_then(|e| Some(categories_iter.zip(e.1.iter())))
    }

    pub fn categories(&'a self) -> impl Iterator<Item=&'a str> {
        self.categories.iter().map(|s| s.as_str())
    }

    pub fn values(&self) -> impl Iterator<Item=(&DateTime<Local>, &Vec<Currency>)> {
        self.data.iter()
    }

    pub fn set_data_for_date<T: Into<Vec<Currency>>>(&mut self, date: DateTime<Local>, data: T) {
        self.data.insert(date, data.into());
    }

    pub fn add_category(&mut self, category: String) {
        self.categories.push(category);
    }

    pub fn add_categories(&mut self, categories: Vec<String>) {
        self.categories = categories;
    }

    pub fn get_latest_value(&self, category: &str) -> Option<Currency> {
        self.data().and_then(|data| {
            for (cat, curr) in data {
                if cat.eq(category) { return Some(*curr); }
            }

            None
        })
    }
}

pub fn get_portfolio_path(portfolio_name: String) -> Result<PathBuf, String> {
    files::get_full_path(portfolio_name).map_or_else(
                |e| { Err(format!("Failed to handle file name: {}", e))},
                |mut f| { f.set_extension("csv"); Ok(f) })
}

pub fn get_portfolio_contents(portfolio_name: String) -> Result<String, String> { 
    let portfolio_path = files::list_data_files().into_iter()
        .find(|f| files::as_file_stem(f).eq(&portfolio_name));

    if let Some(path) = portfolio_path {
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read portfolio file: {}", e))
    } else {
        Err(format!("Portfolio {} wasn't found. Make sure you've spelled it correctly", portfolio_name))
    }
}

pub fn get_portfolio_contents_interactively() -> Result<String, String> {
    let portfolio_path = select_portfolio_file();

    if let Some(path) = portfolio_path {
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read portfolio file: {}", e))
    } else {
        Err(String::from("Didn't find any portfolios"))
    }
}

pub fn get_portfolio(portfolio_name: String) -> Result<Portfolio, String> {
    let portfolio_path = get_portfolio_path(portfolio_name.to_string())?;
    csv::read_portfolio(&portfolio_path)
}

pub fn get_portfolio_interactively(file_name: Option<PathBuf>) -> Result<(Portfolio, PathBuf), String> {
    match file_name {
        Some(name) => {
            if !name.exists() {
                return Err(format!("Portfolio {name} doesn't exist. Try running 'rustfolio new {name}' to create one", 
                                        name = name.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or(String::new())))
            }
            Ok((csv::read_portfolio(&name)?, name))
        },
        None => {
            let name = select_portfolio_file();
            if let Some(name) = name {
                let p = csv::read_portfolio(&name)?;
                Ok((p, name))
            } else {
                Err(String::from("No portfolios exist so far. Try running 'rustfolio new' to create one"))
            }
        },
    }
}

pub fn get_portfolio_name_interactively(portfolio_name: Option<String>) -> Result<String, String> {
    fn validation(s: &String) -> Result<String, String> {
        if s.len() > 0 { Ok(s.clone()) } else { Err(String::from("Portfolio name cannot be empty")) }
    }

    if let Some(name) = portfolio_name {
        Ok(name)
    } else {
        let input = interaction::Input::new("Your portfolio name", validation);
        input.ask_for_input()
    }
}

pub fn read_portfolio_name() -> Result<String, String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)
        .map_err(|e| format!("Failed to read portfolio name: {}", e))?;
    let buffer = String::from(buffer.trim());

    if buffer.len() > 0 { Ok(buffer) } else { Err(String::from("Portfolio name cannot be empty")) }
}

fn select_portfolio_file() -> Option<PathBuf> {
    let mut files = super::files::list_data_files();
    if files.is_empty() {
        return None;
    }

    if files.len() == 1 {
        return Some(files.remove(0));
    }

    Some(interaction::select_one("Select portfolio", files.into_iter(), |f| super::files::as_file_stem(f)))
}
