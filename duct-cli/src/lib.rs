use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "duct")]
#[command(about = "A powerful and observant HTTP client", long_about = None)]
pub struct Cli {
    pub url: String,

    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    #[arg(short, long)]
    pub data: Option<String>,

    #[arg(short = 'H', long)]
    pub header: Vec<String>,

    #[arg(short, long)]
    pub trace: bool,

    #[arg(long)]
    pub user_agent: Option<String>,

    #[arg(short, long)]
    pub redirection: bool,
}
