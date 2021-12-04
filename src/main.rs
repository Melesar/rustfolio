mod interaction;
mod currency;
mod portfolio;
mod csv;
mod add_interactively;
mod show;
mod files;

use args::Args;
use getopts::Occur;

use std::process::exit;

fn main() {
    let mut description = Args::new("rustfolio", "Program for simple portfolio management");
    description.flag("h", "help", "Prints this message");
    description.flag("u", "update", "Updates portfolio with a new entry");
    description.option("f", "file", "Portfolio file", "FILE", Occur::Optional, Some(String::new()));

    if let Err(e) = description.parse_from_cli() {
        eprintln!("Failed to parse command line arguments");
        eprintln!("{}", e);
        exit(1);
    }

    if description.value_of("help").unwrap() {
        println!("{}", description.full_usage());
        return;
    }

    let file_name : Option<String> = description.optional_value_of("file").unwrap_or(None);

    let file_path = file_name
            .and_then(|f| files::get_full_path(f).map_or_else(
                |e| { eprintln!("Failed to handle file name: {}", e); None},
                |mut f| { f.set_extension("csv"); Some(f) }));

    //TODO handle non-tty stdin
    let result = if description.value_of("update").unwrap() {
        add_interactively::add(file_path)
    } else {
        let r = portfolio::get_portfolio_interactively(file_path);
        r.and_then(|p| { 
            if let Some(portfolio) = p {
                show::show_portfolio(&portfolio);
                Ok(()) 
            } else {
                Err(String::from("No portfolios exist so far. Try running rustfolio with -u flag to create one"))
            }
        })
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}

