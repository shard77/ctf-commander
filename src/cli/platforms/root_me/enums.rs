use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum RootMe {
    Auth {
        #[arg(short, long)]
        method: String,
    },
}
