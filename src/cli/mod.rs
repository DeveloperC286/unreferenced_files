use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unreferenced_files",
    about = "A tool for parsing directories scanning all the files within to find unused/unreferenced files."
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
}
