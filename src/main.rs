mod interaction;
mod currency;
mod portfolio;
mod csv;
mod add;
mod show;
mod files;
mod list;
mod export;
mod redirection;
mod new;

use std::path::PathBuf;
use clap::{App, Arg, SubCommand, ArgMatches};
use crossterm::tty::IsTty;

enum DisplayStyle { Chart, Table }

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
        .arg(Arg::with_name("table")
             .help("Display portfolio as a table instead of a chart")
             .long("table"))
        .subcommand(SubCommand::with_name("new")
                    .about("Creates a new portfolio")
                    .display_order(0)
                    .arg(Arg::with_name("portfolio_name")
                         .help("Name of the new portfolio")
                         .value_name("NAME")))
        .subcommand(SubCommand::with_name("add")
                    .about("Adds a new entry to a portfolio")
                    .display_order(1)
                    .arg(file_arg.clone()))
        .subcommand(SubCommand::with_name("list")
                    .about("Lists all available portfolios")
                    .display_order(2))
        .subcommand(SubCommand::with_name("export")
                    .about("Exports a portfolio as a .csv file")
                    .display_order(3)
                    .arg(file_arg.clone())
                    .arg(Arg::with_name("output_file")
                         .short("o")
                         .help("Output file path")
                         .takes_value(true)
                         .required(true)))
        .get_matches();


    let is_stdin_redirected = !std::io::stdin().is_tty();
    let is_stdout_redirected = !std::io::stdout().is_tty();
    let is_tty = !is_stdout_redirected && !is_stdin_redirected;

    let result = if is_tty {
        run_interactively(&app_config) 
    } else {
        redirection::run_redirected(is_stdin_redirected, is_stdout_redirected, &app_config)
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}

fn run_interactively(app_config: &ArgMatches) -> Result<(), String> {

    let display_style = if app_config.is_present("table") { DisplayStyle::Table } else { DisplayStyle::Chart }; 

    if let Some(new_matches) = app_config.subcommand_matches("new") {
        create_new_portfolio(&new_matches)
    } else if let Some(add_matches) = app_config.subcommand_matches("add") {
        let file_path = get_portfolio_path(add_matches);
        add::add_interactively(file_path)
    } else if app_config.subcommand_matches("list").is_some() {
        list::list_portfolio_files(); Ok(())
    } else if let Some(export_matches) = app_config.subcommand_matches("export") {
        export_portfolio(export_matches)
    } else {
        show_portfolio(&app_config, display_style)
    }
}

fn create_new_portfolio(matches: &ArgMatches) -> Result<(), String> {
    let portfolio_name = matches.value_of("portfolio_name");
    let path = portfolio::get_portfolio_name_interactively(portfolio_name.map(|s| s.to_string()))?;
    new::create_portfolio_interactively(path)
}

fn export_portfolio(matches: &ArgMatches) -> Result<(), String> {
    let portfolio_name = matches.value_of("file").map(|s| s.to_string());
    let file_path = std::path::Path::new(matches.value_of("output_file").unwrap());
    export::export_interactively(portfolio_name, file_path)
}

fn show_portfolio(app_config: &ArgMatches, style: DisplayStyle) -> Result<(), String> {
    let file_path = get_portfolio_path(app_config);
    let (portfolio, _) = portfolio::get_portfolio_interactively(file_path)?;
    match style {
        DisplayStyle::Chart => show::show_as_chart(&portfolio),
        DisplayStyle::Table => show::show_as_table(&portfolio),
    }
}

fn get_portfolio_path(matches: &ArgMatches) -> Option<PathBuf> {
    let file_name = matches.value_of("file").map(|s| s.to_string());

    file_name.and_then(|f| portfolio::get_portfolio_path(f).ok())
}

