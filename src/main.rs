mod interaction;
mod currency;
mod portfolio;
mod csv;
mod add_interactively;
mod show;
mod files;

use clap::{App, Arg, SubCommand, crate_version, crate_name, crate_authors, crate_description};

fn main() {
    let app_config = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("add")
             .short("a")
             .long("add")
             .help("Adds a new entry to a portfolio"))
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .help("Portfolio file")
             .value_name("FILE")
             .takes_value(true)
             .required(false))
        .get_matches();

    let file_name = app_config.value_of("file").map(|s| s.to_string());

    let file_path = file_name
            .and_then(|f| files::get_full_path(f).map_or_else(
                |e| { eprintln!("Failed to handle file name: {}", e); None},
                |mut f| { f.set_extension("csv"); Some(f) }));

    //TODO handle non-tty stdin
    let result = if app_config.is_present("add") {
        add_interactively::add(file_path)
    } else {
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

