use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum HackTheBox {
    Auth {
        #[arg(short, long, help = "Authentication method")]
        method: String,
    },
}
