use clap::ArgMatches;

use std::path::Path;
use crate::{add, export, list};

pub fn run_redirected(is_stdin_redirected: bool, is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_redirected(is_stdin_redirected, is_stdout_redirected, add_matches)
    } else if let Some(export_matches) = matches.subcommand_matches("export") {
        export_redirected(&export_matches)
    } else if matches.is_present("list") {
        list_redirected(is_stdout_redirected)
    } else {
        Err(String::from("Only 'add' and 'export' subcommands are supported in non-interactive mode so far"))
    }
}

fn add_redirected(is_stdin_redirected: bool, is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    if is_stdout_redirected && !is_stdin_redirected {
        let mut error_msg = String::from("Sorry, 'rustfolio add' doesn't work with redirected stdout and tty stdin. ");
        error_msg.push_str("Try piping data into it as following:\n");
        error_msg.push_str("category_name_1\namount1\ncategory_name2\namount2\n...");
        return Err(error_msg);
    }

    let file_name = matches
        .value_of("file")
        .ok_or(String::from("--file is required in non-interactive mode"))?
        .to_string();


    add::add_redirected(file_name)
}

fn export_redirected(matches: &ArgMatches) -> Result<(), String> {
    let output_file = matches.value_of("output_file").unwrap();
    let portfolio_name = matches.value_of("file").ok_or(String::from("--file option is required in non-interactive mode"))?;

    export::export_redirected(portfolio_name.to_string(), Path::new(output_file))
}

fn list_redirected(is_stdout_redirected: bool) -> Result<(), String> {
    if is_stdout_redirected {
        list::list_portfolio_files_redirected();
        Ok(())
    } else {
        list::list_portfolio_files(); Ok(())
    }
}
