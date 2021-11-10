use std::path::PathBuf;
use std::io::{BufRead, Write};

use super::csv;
use super::portfolio::Portfolio;

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

    let mut file_list = String::new();
    for (i, file) in files.iter().enumerate() {
        file_list.push_str(&format!("{}. {}\n", i + 1, file.file_stem().unwrap().to_string_lossy()))
    }

    loop {
        println!("{}", file_list);
        print!("Select portfolio to update (number from 1 to {}): ", files.len());

        std::io::stdout().flush().unwrap();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<usize>() {
            Ok(index) => { 
                if index <= files.len() {
                    return Some(files.remove(index - 1));
                } else {
                    println!("Sorry, try again");
                }
            },
            Err(_) => println!("Sorry, try again"),
        }
    }
}

fn ask_for_new_file() -> Result<PathBuf, String> {
    loop {
        print!("Enter your portfolio name: ");
        std::io::stdout().flush().unwrap();
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();
        
        let temp_path = std::path::PathBuf::from(response.trim());
        let stem = temp_path.file_stem();
        if let Some(stem) = stem {
            return super::files::get_full_path(stem)
                .and_then(|mut path| { path.set_extension("csv"); Ok(path) })
                .map_err(|e| e.to_string());
        } else {
            println!("Please enter a valid file name");
        }
    }
}
 

fn read_portfolio (path: &PathBuf) -> Result<Portfolio, String> {
    csv::read_portfolio(path)
}

fn create_new_portfolio(path: PathBuf) {
    print!("A portfolio at {} was not found. Create? [Y/n] ", path.to_string_lossy());
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();

    if response.len() == 1 && response.chars().next().unwrap() != 'y'  { //TODO compare case-insensitive
        return;
    }

    let mut portfolio = Portfolio::new();
    println!("Please enter the categories with their initial values. If omitted, the value will be 0");
    println!("Example: \ncash 100\nbonds 1000\nstocks\n<Ctrl-D>");
    println!("This will make a portfolio as the following:\ncash - 100\nbonds - 1000\nstocks - 0");

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

