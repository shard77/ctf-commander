use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

mod hack_the_box;

#[derive(Subcommand, Debug)]
pub enum Platforms {
    #[command(subcommand, alias = "htb")]
    HackTheBox(hack_the_box::enums::HackTheBox),
    RootMe,
}

pub fn handle_platform(platform: Platforms) {
    match platform {
        Platforms::HackTheBox(subcommand) => hack_the_box::handle(subcommand),
        _ => println!("not implemented yet"),
    }
}
