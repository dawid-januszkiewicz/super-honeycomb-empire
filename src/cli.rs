use clap::{Parser, ValueEnum};

// #[command(name = "example")]
// #[command(about = "An example of Clap usage", long_about = None)]
#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = Mode::Client)]
    pub mode: Mode,

    #[arg(short = 'a', long = "address", default_value = "127.0.0.1:8080")]
    pub addrs: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Client,
    Server,
}
