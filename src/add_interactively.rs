use std::path::PathBuf;

use super::csv;
use super::portfolio::Portfolio;
use super::interaction;

pub fn add (file_name: Option<PathBuf>) -> Result<(), String> {
    let (portfolio, path) = match file_name {
        Some(name) => 
            if name.exists() {
                (Some(read_portfolio(&name)?), name)
            } else {
                (None, name)
            }
        ,
        None => {
            let name = select_portfolio_file();
            if let Some(name) = name {
                let p = Some(read_portfolio(&name)?);
                (p, name)
            } else {
                (None, ask_for_new_file()?)
            }
        },
    };

    if let Some(mut p) = portfolio {
        update_categories(&mut p);
        csv::save_portfolio(&path, p)?;
    } else {
        create_new_portfolio(path);
    }

    Ok(())
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
 

fn read_portfolio (path: &PathBuf) -> Result<Portfolio, String> {
    csv::read_portfolio(path)
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
