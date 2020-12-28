#[macro_use]
extern crate log;
extern crate regex;

use structopt::StructOpt;

mod cli;
mod file_content;
mod file_utilities;
mod regex_utilities;
mod unreferenced_files;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    unreferenced_files::print(
        &arguments.from,
        &arguments.search,
        !arguments.only_file_name && !arguments.only_file_stem,
        !arguments.only_relative_path && !arguments.only_file_stem,
        !arguments.only_relative_path && !arguments.only_file_name,
    );
}
