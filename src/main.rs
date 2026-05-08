use clap::{Parser};
use barst::cmd::init::GitSubcommands;
use barst::cmd::init::Commands;
use barst::cmd::init::Cli;
use barst::config;

fn main() {
    // let _result = config::read_config_file::get_config_file();
    let cli = Cli::parse();

    if cli.verbose {
        println!("HI I'm verbose")
    }

    match &cli.command {
        Commands::Context(git_subs) => match git_subs {
            GitSubcommands::Git { create  , collect: _, cmd: _,  status: _   } => println!("Creating storage file... {}", create.to_string()),
        },
        Commands::Config {} => {
            let vals = config::read_config_file::config_values();
            println!("Priting Config values... {}", vals.db_url);
        },
    }
}
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}