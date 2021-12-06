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

    pub fn get_latest_value(&self, category: &str) -> Option<Currency> {
        self.data().and_then(|data| {
            for (cat, curr) in data {
                if cat.eq(category) { return Some(*curr); }
            }

            None
        })
    }
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

pub fn create_portfolio_interactively(file_name: PathBuf) -> Result<Portfolio, String> {
    if file_name.exists() {
        Err(String::from("Portfolio with provided file_name already exists"))
    } else {
        let mut portfolio = Portfolio::new();
        interaction::populate_new_portfolio(&mut portfolio);
        csv::save_portfolio(&file_name, &portfolio).map_err(|e| format!("Failed to save portfolio: {}", e))?;
        Ok(portfolio)
    }
}

pub fn get_portfolio_name_interactively(portfolio_name: Option<String>) -> Result<PathBuf, String> {
    fn validation(s: &String) -> Result<String, String> {
        Ok(String::from(s))
    }

    let portfolio_name = portfolio_name.unwrap_or_else(|| {
        let input = interaction::Input::new("Your portfolio name", validation);
        input.ask_for_input().unwrap_or(String::new())
    });

    let temp_path = std::path::PathBuf::from(portfolio_name.trim());
    match temp_path.file_stem() {
        Some(stem) => {
            super::files::get_full_path(stem)
                .and_then(|mut path| { path.set_extension("csv"); Ok(path) })
                .map_err(|e| e.to_string())
        },
        None => Err(String::from("Invalid portfolio name")),
    }
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
