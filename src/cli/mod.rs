use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unreferenced_files",
    about = "A tool for parsing directories and scanning all the files within, to find unused and unreferenced files.",
    group = ArgGroup::with_name("only").required(false)
)]
pub struct Arguments {
    #[structopt(
        short,
        long,
        help = "Determine if the files in this directory are referenced."
    )]
    pub from: String,

    #[structopt(
        short,
        long,
        help = "The directory of files to scan for references to the files."
    )]
    pub search: String,

    #[structopt(
        group = "only",
        long,
        help = "Only search for unreferenced files via their file name."
    )]
    pub only_file_name: bool,

    #[structopt(
        group = "only",
        long,
        help = "Only search for unreferenced files via their file name without the extension."
    )]
    pub only_file_stem: bool,

    #[structopt(
        group = "only",
        long,
        help = "Only search for unreferenced files via their relative path."
    )]
    pub only_relative_path: bool,
}
