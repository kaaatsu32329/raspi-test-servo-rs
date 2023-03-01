use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short = "l", long = "min-microseconds", default_value = "1200")]
    pub min_microseconds: u64,
    #[structopt(short = "h", long = "max-microseconds", default_value = "2500")]
    pub max_microseconds: u64,
}
