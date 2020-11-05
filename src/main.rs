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
    info!("{:?}", arguments);

    unreferenced_files::print(&arguments.from, &arguments.search, true, true);
}
