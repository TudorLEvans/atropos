use clap::{Parser, Subcommand, Args};


#[derive(Parser, Debug)]
#[command(author = "Tudor Evans", version = "0.1.0", about = "Rust implementation of a basic metronome", long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Met(Metronome)
}


#[derive(Args, Debug)]
pub struct Metronome {
    #[arg(short, long, default_value_t = 4)]
    pub bar_length: u32,
    #[arg(short, long, default_value_t = 1)]
    pub sub_divisions: u32,
    #[arg(short, long, default_value_t = 100, value_parser = validate_tempo)]
    pub tempo: u32,
    #[arg(short, long, default_value_t = false)]
    pub use_bell: bool
}

fn validate_tempo(s: &str) -> Result<u32, String> {
    let tempo: u32 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if tempo > 0 {
        Ok(tempo)
    } else {
        Err(String::from("Tempo must be greater than 0"))
    }
}