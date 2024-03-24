mod platforms;
mod utils;

use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "ctf-commander",
    version,
    about = "Interact with your favorite CTF platforms through the terminal!",
    help_template = "\
{before-help}{name} v{version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Cli {
    #[command(subcommand)]
    platform: Option<platforms::Platforms>,
}

pub fn run() {
    let args = Cli::parse();

    match args.platform {
        Some(platform) => platforms::handle_platform(platform),
        None => println!("No platform specified, or show general help or commands."),
    }
}
