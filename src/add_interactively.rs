use std::path::PathBuf;
use std::io::{BufRead, Write};
use super::portfolio::Portfolio;

pub fn add (file_name: Option<PathBuf>) -> Result<(), String> {
    //TODO handle non-tty stdin
    println!("Add fn");
    let (portfolio, path) = match file_name {
        Some(name) => (read_portfolio(&name), name),
        None => {
            let name = promt_file_name();
            let p = read_portfolio(&name);
            (p, name)
        },
    };

    println!("Action");
    if let Some(p) = portfolio {
        update_categories(p);
    } else {
        create_new_portfolio(path);
    }

    Ok(())
}

fn promt_file_name() -> PathBuf {
    PathBuf::new()
}

fn read_portfolio (path: &PathBuf) -> Option<Portfolio> {
    None
}

fn create_new_portfolio(path: PathBuf) {
    print!("A portfolio at {} was not found. Create? [Y/n] ", path.to_string_lossy());
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();

    if response.len() == 1 && response.chars().next().unwrap() == 'y'  { //TODO compare case-insensitive
        return;
    }

    let mut portfolio = Portfolio::new(path);
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

    portfolio.save();

    println!("Your new portfolio:\n{}", portfolio);
}

fn update_categories(mut portfolio: Portfolio) {
    for (category, current_amount) in portfolio.categories_mut() {
        loop {
            print!("Amount for {}: ", category);
            let mut amount = String::new();
            std::io::stdin().read_line(&mut amount).unwrap();

            let error_msg = "Amount must be a positive floating point number";
            let amount = amount.trim()
                .parse::<f32>()
                .map_err(|_| String::from(error_msg))
                .and_then(|f| if f >= 0.0 { Ok(f) } else { Err(String::from(error_msg)) });

            match amount {
                Ok(f) => { *current_amount = f; break },
                Err(e) => {
                    eprintln!("{}", e);
                    eprintln!("Sorry, try again");
                }
            }
        }
    }
}

