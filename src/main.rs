mod portfolio;
mod add_interactively;
mod show;
mod files;

use args::Args;
use getopts::Occur;

use std::process::exit;

fn main() {
    let mut description = Args::new("rustfolio", "Program for simple portfolio management");
    description.flag("h", "help", "Prints this message");
    description.flag("a", "add", "Updates portfolio with a new entry");
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
    let result = if description.value_of("add").unwrap() {
        add_interactively::add(file_name
            .and_then(|f| files::get_full_path(f).map_or_else(
                |e| { eprintln!("Failed to handle file name: {}", e); None},
                |f| Some(f)))
        )
    } else {
        Ok(())
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}

