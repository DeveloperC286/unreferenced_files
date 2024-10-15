#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use anyhow::{bail, Result};
use clap::Parser;

use crate::cli::Arguments;
use crate::filters::Filters;
use crate::search::Search;
use crate::search_for::SearchFor;

mod cli;
mod file_path_variants;
mod file_path_variants_regexes;
mod filters;
mod reporter;
mod search;
mod search_for;
mod utilities;

#[cfg(test)]
mod tests;

// TODO Assert failed vs failed to do something error status codes.
const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::parse();
    debug!("The command line arguments provided are {:?}.", arguments);

    if let Err(err) = run(arguments) {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
    let search_for_relative_path = !arguments.only_file_name && !arguments.only_file_stem;
    let search_for_file_name = !arguments.only_relative_path && !arguments.only_file_stem;
    let search_for_file_stem = !arguments.only_relative_path && !arguments.only_file_name;

    let search_for_fitlers = Filters::new(arguments.only_search_for, arguments.ignore_search_for)?;
    let search_for = SearchFor::new(&arguments.search_for, search_for_fitlers)?;

    let search_filters = Filters::new(arguments.only_search, arguments.ignore_search)?;
    let search = Search::new(&arguments.search, search_filters)?;

    let unreferenced_files = search_for.get_unreferenced_files(
        search,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    let is_unreferenced_files = !unreferenced_files.is_empty();

    crate::reporter::print(unreferenced_files, arguments.print_full_path);

    if arguments.assert_no_unreferenced_files && is_unreferenced_files {
        bail!("There are unreferenced files.")
    }

    Ok(())
}
