use std::collections::HashMap;
use std::path::PathBuf;

use super::{csv, interaction};
use super::currency::Currency;

pub struct Portfolio {
    data: HashMap<String, Currency>,
}

impl<'a> Portfolio {
    pub fn new() -> Self {
        Portfolio { data: HashMap::new() }
    }

    pub fn data_mut (&'a mut self) -> impl Iterator<Item=(&'a String, &'a mut Currency)> {
        self.data.iter_mut()
    }

    pub fn data(&'a self) -> impl Iterator<Item=(&'a String, &'a Currency)> {
        self.data.iter()
    }

    pub fn categories(&'a self) -> impl Iterator<Item=&'a String> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&Currency> {
        self.data.values()
    }

    pub fn add_category(&mut self, category: String, initial_value: f32) {
        self.data.insert(category, Currency(initial_value));
    }
}

impl std::fmt::Display for Portfolio {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (category, value) in self.data.iter() {
            write!(f, "{} - {}\n", category, value)?;
        }

        Ok(())
    }
}

pub fn get_portfolio_interactively(file_name: Option<PathBuf>) -> Result<(Option<Portfolio>, PathBuf), String> {
    match file_name {
        Some(name) => 
            if name.exists() {
                Ok((Some(csv::read_portfolio(&name)?), name))
            } else {
                Ok((None, name))
            }
        ,
        None => {
            let name = select_portfolio_file();
            if let Some(name) = name {
                let p = Some(csv::read_portfolio(&name)?);
                Ok((p, name))
            } else {
                Ok((None, ask_for_new_file()?))
            }
        },
    }
}

fn select_portfolio_file() -> Option<PathBuf> {
    let mut files = super::files::list_data_files();
    files.retain(|f| f.file_name().is_some());
    if files.is_empty() {
        return None;
    }

    if files.len() == 1 {
        return Some(files.remove(0));
    }

    interaction::select_one("Select portfolio", files.into_iter(), |f| f.file_stem().map_or(String::new(), |stem| stem.to_string_lossy().to_string()))
}

fn ask_for_new_file() -> Result<PathBuf, String> {

    fn validation(s: &String) -> Result<String, String> {
        Ok(String::from(s))
    }

    let portfolio_name = interaction::ask_for_input("Your portfolio name", validation);
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
