use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long)]
    pub word: Option<String>,
    #[clap(short, long)]
    pub random: bool, 
    #[clap(short = 'D', long)]
    pub difficult: bool, 
    #[clap(short = 't', long)]
    pub stats: bool, 
    #[clap(short = 's', long)]
    pub seed: Option<u64>, 
    #[clap(short = 'd', long)]
    pub day: Option<i32>,
    #[clap(short = 'f', long = "final-set")]
    pub final_set: Option<String>,
    #[clap(short = 'a', long = "acceptable-set")]
    pub acceptable_set: Option<String>,
    #[clap(short = 'S', long)]
    pub state: Option<String>,
    #[clap(short, long)]
    pub config: Option<String>,

    #[clap(long)]
    pub hint: bool,
}