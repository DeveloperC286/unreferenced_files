#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use std::process::exit;

use structopt::StructOpt;

mod cli;
mod file_utilities;
mod model;
mod regex_utilities;
mod reporter;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let search_for_relative_path = !arguments.only_file_name && !arguments.only_file_stem;
    let search_for_file_name = !arguments.only_relative_path && !arguments.only_file_stem;
    let search_for_file_stem = !arguments.only_relative_path && !arguments.only_file_name;

    let search_for_filters = crate::model::filters::Filters::new(arguments.only_search_for);
    let mut unreferenced_files = crate::model::unreferenced_files::UnreferencedFiles::new(
        file_utilities::get_paths(arguments.search_for),
        search_for_filters,
    );

    let search_filters = crate::model::filters::Filters::new(arguments.only_search);
    let search = crate::model::raw_files::RawFiles::new(
        file_utilities::get_paths(arguments.search),
        search_filters,
    );

    unreferenced_files.remove_referenced_files(
        search,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    let is_not_empty = !unreferenced_files.is_empty();

    crate::reporter::print(unreferenced_files, arguments.print_full_path);

    if arguments.assert_no_unreferenced_files && is_not_empty {
        exit(ERROR_EXIT_CODE);
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
