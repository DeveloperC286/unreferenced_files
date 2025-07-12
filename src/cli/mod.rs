use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(
        long,
        required = true,
        num_args = 1,
        help = "Search for references of this file or if it is a directory all resources within, multiple resources can be provided."
    )]
    pub(crate) search_for: Vec<String>,

    #[arg(
        long,
        group = "search_for_filters",
        help = "Only search for files that match any of these regexes, mutually exclusive with ignore search for."
    )]
    pub(crate) only_search_for: Vec<String>,

    #[arg(
        long,
        group = "search_for_filters",
        help = "Ignore and do not search for any files that match any of these regexes, mutually exclusive with only search for."
    )]
    pub(crate) ignore_search_for: Vec<String>,

    #[arg(
        long,
        required = true,
        num_args = 1,
        help = "Search this file or if it is a directory all resources within for references, multiple resources can be provided."
    )]
    pub(crate) search: Vec<String>,

    #[arg(
        long,
        group = "search_filters",
        help = "Only search files that match any of these regexes, mutual exclusive with ignore search."
    )]
    pub(crate) only_search: Vec<String>,

    #[arg(
        long,
        group = "search_filters",
        help = "Ignore and do not search any files that match any of these regexes, mutually exclusive with only search."
    )]
    pub(crate) ignore_search: Vec<String>,

    #[arg(
        long,
        group = "only",
        help = "Only search for unreferenced files via their file name, mutually exclusive with other only flags."
    )]
    pub(crate) only_file_name: bool,

    #[arg(
        long,
        group = "only",
        help = "Only search for unreferenced files via their file name without the extension, mutually exclusive with other only flags."
    )]
    pub(crate) only_file_stem: bool,

    #[arg(
        long,
        group = "only",
        help = "Only search for unreferenced files via their relative path, mutually exclusive with other only flags."
    )]
    pub(crate) only_relative_path: bool,

    #[arg(
        long,
        help = "Output the full path of each unreferenced file, instead of the relative path."
    )]
    pub(crate) print_full_path: bool,

    #[arg(
        long,
        help = "Return a nonzero exit code if there are any unreferenced files."
    )]
    pub(crate) assert_no_unreferenced_files: bool,

    #[arg(
        long,
        help = "Enable verbose output, respects RUST_LOG environment variable if set."
    )]
    pub(crate) verbose: bool,
}
