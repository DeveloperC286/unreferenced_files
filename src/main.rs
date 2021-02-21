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
mod unreferenced_files;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let search_for_relative_path = !arguments.only_file_name && !arguments.only_file_stem;
    let search_for_file_name = !arguments.only_relative_path && !arguments.only_file_stem;
    let search_for_file_stem = !arguments.only_relative_path && !arguments.only_file_name;

    let search_for = crate::model::file_path_variants::get_file_path_variants(
        file_utilities::get_paths(arguments.search_for),
        arguments.from_ignore_file_regex,
    );
    let searching = crate::model::raw_file::get_raw_files(
        file_utilities::get_paths(arguments.search),
        arguments.search_ignore_file_regex,
    );

    let unreferenced_files = crate::unreferenced_files::get_unreferenced_files(
        search_for,
        searching,
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
