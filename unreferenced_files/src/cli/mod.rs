use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "search_for",
    about = "A utility for finding unused and unreferenced files.",
    group = ArgGroup::with_name("only").required(false),
    group = ArgGroup::with_name("search_filters").required(false),
    group = ArgGroup::with_name("search_for_filters").required(false),
)]
pub(crate) struct Arguments {
    #[structopt(
        long,
        required = true,
        min_values = 1,
        help = "Search for references of this file or if it is a directory all resources within, multiple resources can be provided."
    )]
    pub(crate) search_for: Vec<String>,

    #[structopt(
        long,
        group = "search_for_filters",
        help = "Only search for files that match any of these regexes, mutually exclusive with ignore search for."
    )]
    pub(crate) only_search_for: Vec<String>,

    #[structopt(
        long,
        group = "search_for_filters",
        help = "Ignore and do not search for any files that match any of these regexes, mutually exclusive with only search for."
    )]
    pub(crate) ignore_search_for: Vec<String>,

    #[structopt(
        long,
        required = true,
        min_values = 1,
        help = "Search this file or if it is a directory all resources within for references, multiple resources can be provided."
    )]
    pub(crate) search: Vec<String>,

    #[structopt(
        long,
        group = "search_filters",
        help = "Only search files that match any of these regexes, mutual exclusive with ignore search."
    )]
    pub(crate) only_search: Vec<String>,

    #[structopt(
        long,
        group = "search_filters",
        help = "Ignore and do not search any files that match any of these regexes, mutually exclusive with only search."
    )]
    pub(crate) ignore_search: Vec<String>,

    #[structopt(
        long,
        group = "only",
        help = "Only search for unreferenced files via their file name, mutually exclusive with other only flags."
    )]
    pub(crate) only_file_name: bool,

    #[structopt(
        long,
        group = "only",
        help = "Only search for unreferenced files via their file name without the extension, mutually exclusive with other only flags."
    )]
    pub(crate) only_file_stem: bool,

    #[structopt(
        long,
        group = "only",
        help = "Only search for unreferenced files via their relative path, mutually exclusive with other only flags."
    )]
    pub(crate) only_relative_path: bool,

    #[structopt(
        long,
        help = "Output the full path of each unreferenced file, instead of the relative path."
    )]
    pub(crate) print_full_path: bool,

    #[structopt(
        long,
        help = "Return a nonzero exit code if there are any unreferenced files."
    )]
    pub(crate) assert_no_unreferenced_files: bool,
}
