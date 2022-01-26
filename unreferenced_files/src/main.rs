#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::process::exit;

use structopt::StructOpt;
use unreferenced_files_lib::filters::Filters;
use unreferenced_files_lib::search::Search;
use unreferenced_files_lib::search_for::SearchFor;

mod cli;
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

    let search_for = match Filters::new(arguments.only_search_for, arguments.ignore_search_for) {
        Ok(filters) => match SearchFor::new(&arguments.search_for, filters) {
            Ok(search_for) => search_for,
            Err(_) => {
                exit(ERROR_EXIT_CODE);
            }
        },
        Err(_) => {
            exit(ERROR_EXIT_CODE);
        }
    };

    let search = match Filters::new(arguments.only_search, arguments.ignore_search) {
        Ok(filters) => match Search::new(&arguments.search, filters) {
            Ok(search) => search,
            Err(_) => {
                exit(ERROR_EXIT_CODE);
            }
        },
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
