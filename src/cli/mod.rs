use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "unreferenced_files")]
pub struct Arguments {
    #[structopt(long = "from", help = "")]
    pub from: String,
    #[structopt(long = "search", help = "")]
    pub search: String,
}
