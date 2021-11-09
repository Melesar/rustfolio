use std::path::PathBuf;
use super::portfolio::Portfolio;

pub fn add (file_name: Option<PathBuf>) -> Result<(), String> {
    //TODO handle non-tty stdin

    let mut portfolio = match file_name {
        Some(name) => read_portfolio(name),
        None => promt_file_name().and_then(|f| read_portfolio(f)),
    }?;

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

    Ok(())
}

fn promt_file_name() -> Result<PathBuf, String> {
    Err(String::new())
}

fn read_portfolio (path: PathBuf) -> Result<Portfolio, String> {
    Err(String::new())
}

