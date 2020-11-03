use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unreferenced_files",
    about = "A tool for parsing directories scanning all the files within to find unused/unreferenced files."
)]
pub struct Arguments {
    #[structopt(long = "from", help = "")]
    pub from: String,
    #[structopt(long = "search", help = "")]
    pub search: String,
}
