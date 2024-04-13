use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum HackTheBox {
    Auth,
    // todo: find a way to pass arg without having to implicitly declare it with a flag
    GetMachineProfile {
        #[arg(short, long)]
        name: String,
    },
    ListMachines,
}
