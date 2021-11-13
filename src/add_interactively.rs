use std::path::PathBuf;
use std::io::{BufRead, Write};

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

    interaction::select_one("Select portfolio", files.into_iter())
}

fn ask_for_new_file() -> Result<PathBuf, String> {
    let portfolio_name = interaction::ask_for_input("Your portfolio name");
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
    if !interaction::confirmation(&confirmation_label) { 
        return;
    }

    let mut portfolio = Portfolio::new();
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut words = line.split(" ");
        let category = words.next();
        if category.is_none() { continue; }

        let value = words.next().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
        portfolio.add_category(category.unwrap().to_string(), value);
    }

    println!("Your new portfolio:\n{}", portfolio);
    
    if let Err(s) = csv::save_portfolio(&path, portfolio) {
        eprintln!("{}", s);
    }
}

fn update_categories(portfolio: &mut Portfolio) {
    for (category, current_amount) in portfolio.data_mut() {
        loop {
            print!("Amount for {}: ", category);
            std::io::stdout().flush().unwrap();
            let mut amount = String::new();
            std::io::stdin().read_line(&mut amount).unwrap();

            let error_msg = "Amount must be a positive floating point number";
            let amount = amount.trim()
                .parse::<f32>()
                .map_err(|_| String::from(error_msg))
                .and_then(|f| if f >= 0.0 { Ok(f) } else { Err(String::from(error_msg)) });

            match amount {
                Ok(f) => { **current_amount = f; break },
                Err(e) => {
                    eprintln!("{}", e);
                    eprintln!("Sorry, try again");
                }
            }
        }
    }

}

