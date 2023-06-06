use clap::{Parser, Subcommand};

pub struct Metronome {
    pub bar_length: u32,
    pub sub_divisions: u32,
    pub tempo: u32,
    pub use_bell: bool,
    pub use_sub: bool
}

#[derive(Parser, Debug)]
#[command(author = "Tudor Evans", version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Met {
        #[clap()]
        bar_length: u32,
        #[clap()]
        sub_divisions: u32,
        // #[clap(validator = validate_tempo, default_value_t = 100)]
        tempo: u32,
        #[clap(short, long, default_value_t = true)]
        use_bell: bool,
        #[clap(short, long, default_value_t = false)]
        use_sub: bool
    }
}

fn validate_tempo(tempo: &u32) -> Result<(), String> {
    if *tempo < 1 {
        Err(String::from(
            "Tempo must be a positive integer",
        ))
    } else {
        Ok(())
    }
}