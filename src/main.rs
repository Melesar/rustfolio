mod interaction;
mod currency;
mod portfolio;
mod csv;
mod add_interactively;
mod show;
mod files;

use std::path::PathBuf;
use clap::{App, Arg, SubCommand, ArgMatches};

fn main() {
    let file_arg = Arg::with_name("file")
        .short("f")
        .long("file")
        .help("Portfolio file")
        .value_name("FILE")
        .takes_value(true)
        .required(false);

    let app_config = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(file_arg.clone())
        .subcommand(SubCommand::with_name("add")
             .about("Adds a new entry to a portfolio")
             .arg(file_arg.clone()))
        .get_matches();


    //TODO handle non-tty stdin
    let result = if let Some(add_matches) = app_config.subcommand_matches("add") {
        let file_path = get_file_name(add_matches);
        add_interactively::add(file_path)
    } else {
        let file_path = get_file_name(&app_config);
        let r = portfolio::get_portfolio_interactively(file_path);
        r.and_then(|p| { 
            if let Some(portfolio) = p {
                show::show_portfolio(&portfolio)
            } else {
                Err(String::from("No portfolios exist so far. Try running rustfolio with -a or --add flag to create one"))
            }
        })
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}

fn get_file_name(matches: &ArgMatches) -> Option<PathBuf> {
    let file_name = matches.value_of("file").map(|s| s.to_string());

    file_name.and_then(|f| files::get_full_path(f).map_or_else(
                |e| { eprintln!("Failed to handle file name: {}", e); None},
                |mut f| { f.set_extension("csv"); Some(f) }))
}

