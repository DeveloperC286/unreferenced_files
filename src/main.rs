#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use std::process::exit;

use structopt::StructOpt;

mod cli;
mod model;
mod reporter;

// TODO Assert failed vs failed to do something error status codes.
const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = crate::cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let search_for_relative_path = !arguments.only_file_name && !arguments.only_file_stem;
    let search_for_file_name = !arguments.only_relative_path && !arguments.only_file_stem;
    let search_for_file_stem = !arguments.only_relative_path && !arguments.only_file_name;

    let search_for = match crate::model::filters::Filters::new(
        arguments.only_search_for,
        arguments.ignore_search_for,
    ) {
        Ok(filters) => crate::model::search_for::SearchFor::new(&arguments.search_for, filters),
        Err(_) => {
            exit(ERROR_EXIT_CODE);
        }
    };

    let search =
        match crate::model::filters::Filters::new(arguments.only_search, arguments.ignore_search) {
            Ok(filters) => crate::model::search::Search::new(&arguments.search, filters),
            Err(_) => {
                exit(ERROR_EXIT_CODE);
            }
        };

    let unreferenced_files = search_for.get_unreferenced_files(
        search,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    let is_unreferenced_files = !unreferenced_files.is_empty();

    crate::reporter::print(unreferenced_files, arguments.print_full_path);

    if arguments.assert_no_unreferenced_files && is_unreferenced_files {
        exit(ERROR_EXIT_CODE);
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests;
