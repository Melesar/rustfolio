mod interaction;
mod currency;
mod portfolio;
mod csv;
mod add_interactively;
mod show;
mod files;
mod list;
mod export;

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
        .subcommand(SubCommand::with_name("list").about("Lists all available portfolios"))
        .subcommand(SubCommand::with_name("export")
                    .about("Exports a portfolio as a .csv file")
                    .arg(Arg::with_name("portfolio_name")
                         .short("p")
                         .long("portfolio")
                         .help("Name of the portfolio to export")
                         .required(false)
                         .takes_value(true))
                    .arg(Arg::with_name("output_file")
                         .short("o")
                         .help("Output file path")
                         .takes_value(true)
                         .required(true)))
        .get_matches();


    //TODO handle non-tty stdin
    let result = if let Some(add_matches) = app_config.subcommand_matches("add") {
        let file_path = get_file_name(add_matches);
        add_interactively::add(file_path)
    } else if app_config.subcommand_matches("list").is_some() {
        list::list_portfolio_files()
    } else if let Some(export_matches) = app_config.subcommand_matches("export") {
        export_portfolio(export_matches)
    } else {
        show_portfolio(&app_config)
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}

fn export_portfolio(matches: &ArgMatches) -> Result<(), String> {
    let portfolio_name = matches.value_of("portfolio_name").map(|s| s.to_string());
    let file_path = std::path::Path::new(matches.value_of("output_file").unwrap());
    export::export_interactively(portfolio_name, file_path)
}

fn show_portfolio(app_config: &ArgMatches) -> Result<(), String> {
    let file_path = get_file_name(app_config);
    let r = portfolio::get_portfolio_interactively(file_path);
    r.and_then(|p| { 
        if let Some(portfolio) = p {
            show::show_portfolio(&portfolio)
        } else {
            Err(String::from("No portfolios exist so far. Try running rustfolio with -a or --add flag to create one"))
        }
    })
}

fn get_file_name(matches: &ArgMatches) -> Option<PathBuf> {
    let file_name = matches.value_of("file").map(|s| s.to_string());

    file_name.and_then(|f| files::get_full_path(f).map_or_else(
                |e| { eprintln!("Failed to handle file name: {}", e); None},
                |mut f| { f.set_extension("csv"); Some(f) }))
}

