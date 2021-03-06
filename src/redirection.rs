use clap::ArgMatches;

use std::collections::HashMap;
use std::path::Path;

use crate::{add, export, list, portfolio, show, new};

pub fn run_redirected(is_stdin_redirected: bool, is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    if let Some(add_matches) = matches.subcommand_matches("add") {
        add(is_stdin_redirected, is_stdout_redirected, add_matches)
    } else if let Some(new_matches) = matches.subcommand_matches("new") {
        new(is_stdin_redirected, new_matches)
    } else if let Some(export_matches) = matches.subcommand_matches("export") {
        export(&export_matches)
    } else if matches.is_present("list") {
        list(is_stdout_redirected)
    } else {
        show(is_stdout_redirected, matches)
    }
}

pub fn collect_portfolio_data() -> HashMap<String, f32> {
    let stdin = std::io::stdin();
    let mut buffer = String::new();

    let mut current_category : Option<String> = None;
    let mut update_table = HashMap::new();
    while stdin.read_line(&mut buffer).unwrap_or(0) > 0 {
        if let Some(category) = current_category.as_ref() {
            let amount = add::validate_amount(&buffer.trim().to_string());

            if amount.is_err() { 
                current_category = None;
                buffer.clear();
                continue;
            }

            update_table.insert(category.clone(), amount.unwrap());
            current_category = None;
        } else {
            current_category = Some(buffer.trim().to_string());
        }

        buffer.clear();
    }

    update_table
}

fn show(is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    let file_name = matches.value_of("file").ok_or(String::from("--file option is required in non-interactive mode"))?;
    let is_table = matches.is_present("table");
    
    if is_stdout_redirected && !is_table {
        println!("{}", portfolio::get_portfolio_contents(file_name.to_string())?);
        Ok(())
    } else if is_stdout_redirected {
        show::show_as_table(&portfolio::get_portfolio(file_name.to_string())?)
    } else {
        show::show_as_chart(&portfolio::get_portfolio(file_name.to_string())?)
    }
}

fn new(is_stdin_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    if !is_stdin_redirected {
        let mut error_msg = String::from("Sorry, 'rustfolio new' doesn't work with redirected stdout and tty stdin. ");
        error_msg.push_str("Try piping data into it as following:\n");
        error_msg.push_str("category_name_1\namount1\ncategory_name2\namount2\n...");
        return Err(error_msg);
    }

    let should_read_name = matches.is_present("read_name");
    let portfolio_name = if let Some(name) = matches.value_of("portfolio_name") {
        if !should_read_name {
            Ok(String::from(name))
        } else {
            Err(String::from("Warning: the <NAME> argument and --read-name flag may not work together correctly. Aborting"))
        }
    } else if should_read_name {
        portfolio::read_portfolio_name()
    } else {
        Err(String::from("Either <NAME> argument or --read-name flag are required in non-interactive mode"))
    }?;

    let portfolio_path = portfolio::get_portfolio_path(portfolio_name.to_string())?;
    new::create_portfolio_redirected(portfolio_path)
}

fn add(is_stdin_redirected: bool, is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
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

fn export(matches: &ArgMatches) -> Result<(), String> {
    let output_file = matches.value_of("output_file").unwrap();
    let portfolio_name = matches.value_of("file").ok_or(String::from("--file option is required in non-interactive mode"))?;

    export::export_redirected(portfolio_name.to_string(), Path::new(output_file))
}

fn list(is_stdout_redirected: bool) -> Result<(), String> {
    if is_stdout_redirected {
        list::list_portfolio_files_redirected();
        Ok(())
    } else {
        list::list_portfolio_files(); Ok(())
    }
}
