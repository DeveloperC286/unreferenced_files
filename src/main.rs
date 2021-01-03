#[macro_use]
extern crate log;
extern crate regex;

use structopt::StructOpt;

mod cli;
mod file_utilities;
mod model;
mod regex_utilities;
mod reporter;
mod unreferenced_files;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let search_for_relative_path = !arguments.only_file_name && !arguments.only_file_stem;
    let search_for_file_name = !arguments.only_relative_path && !arguments.only_file_stem;
    let search_for_file_stem = !arguments.only_relative_path && !arguments.only_file_name;

    let searching_for = crate::model::file_path_variants::get_file_path_variants(
        file_utilities::get_path(&arguments.from),
        arguments.from_ignore_file_regex,
    );
    let searching = crate::model::raw_file::get_raw_files(
        file_utilities::get_path(&arguments.search),
        arguments.search_ignore_file_regex,
    );
    let searching_for_regex_map = crate::regex_utilities::get_regex_map(
        &searching_for,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    let unreferenced_files = crate::unreferenced_files::get_unreferenced_files_in_directory(
        searching_for,
        searching_for_regex_map,
        searching,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );
    crate::reporter::print(unreferenced_files, arguments.print_full_path);
}
