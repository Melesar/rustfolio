use clap::ArgMatches;

use crate::add;

pub fn run_redirected(is_stdin_redirected: bool, is_stdout_redirected: bool, matches: &ArgMatches) -> Result<(), String> {
    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_redirected(is_stdin_redirected, is_stdout_redirected, add_matches)
    } else  {
        Err(String::from("Only 'add' subcommand is supported in non-interactive mode so far"))
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
