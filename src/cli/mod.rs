use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unreferenced_files",
    about = "A utility for finding unused and unreferenced files.",
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
        long,
        help = "Ignore every file in the from directory that matches any of these regexes."
    )]
    pub from_ignore_file_regex: Vec<String>,

    #[structopt(
        short,
        long,
        help = "The directory of files to scan for references to the files."
    )]
    pub search: String,

    #[structopt(
        long,
        help = "Ignore every file in the search directory that matches any of these regexes."
    )]
    pub search_ignore_file_regex: Vec<String>,

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

    #[structopt(
        long,
        help = "Output the full path of each unreferenced file, instead of the relative path."
    )]
    pub print_full_path: bool,

    #[structopt(
        long,
        help = "Return a non zero exit code if there are any unreferenced files."
    )]
    pub assert_no_unreferenced_files: bool,
}
