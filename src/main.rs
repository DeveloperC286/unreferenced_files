#[macro_use]
extern crate log;
extern crate regex;

use structopt::StructOpt;

mod cli;
mod file_content;
mod file_utilities;
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

    let files = file_utilities::get_files_in_directory(file_utilities::get_path(&arguments.from));
    let regex_map = regex_utilities::get_regex_map(
        &files,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );
    let unreferenced_files = unreferenced_files::get_unreferenced_files_in_directory(
        &files,
        file_utilities::get_path(&arguments.search),
        &regex_map,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );
    reporter::print(unreferenced_files);
}
