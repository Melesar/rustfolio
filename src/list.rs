use std::io::Write;

use super::files;
use crossterm::{
    style::{Print, SetAttribute, Attribute},
    queue,
};

pub fn list_portfolio_files() {
    let files : Vec<String> = files::list_data_files()
        .into_iter()
        .map(|f| files::as_file_stem(&f))
        .collect();

    if files.is_empty() {
        println!("You have no portfolios yet");
    }

    let num_items = files.len();
    let mut stdout = std::io::stdout();
    queue!(stdout, SetAttribute(Attribute::Bold)).unwrap_or_default();
    queue!(stdout, Print(format!("You have {} portfolios:\n", num_items))).unwrap_or_default();
    queue!(stdout, SetAttribute(Attribute::Reset)).unwrap_or_default();
    stdout.flush().unwrap_or_default();

    for (idx, portfolio_name) in files.into_iter().enumerate() {
        println!("  {}. {}", idx + 1, portfolio_name);
    }
}

pub fn list_portfolio_files_redirected() {
    
    let files : Vec<String> = files::list_data_files()
        .into_iter()
        .map(|f| files::as_file_stem(&f))
        .collect();

    if files.is_empty() {
        return;
    }

    for file_name in files {
        println!("{}", file_name);
    }
}
