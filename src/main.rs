use clap::{Parser, Subcommand};
use barst::config;
/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "BARST")]
#[command(version)]
#[command(
    about = "
___  ____ ____ ____ ___
|__] |__| |__/ [__   |
|__] |  | |  \\ ___]  |
", 
    long_about = "
___  ____ ____ ____ ___
|__] |__| |__/ [__   |
|__] |  | |  \\ ___]  |

I am a little weird, and my unconventional ways require me to make things like this
so that I can stay alive as a functional human being.

Stay Spicy, bois!
    ",
)]
struct Cli {
    /// Enable verbose logging mode
    #[arg(short('v'), long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Context for sessions
    #[command(subcommand)]
    Context(GitSubcommands),

    Config {
        /// Check config values
        #[arg(short, long)]
        status: bool,
    },
}

#[derive(Subcommand)]
enum GitSubcommands {
    /// Manages Context for Git
    Git {
        /// Create storage file for gitshit later :3
        #[arg(long)]
        create: bool,

        /// Collect local git repo dir to do stuff to it later :3
        #[arg(short, long)]
        collect: String,

        /// run git status
        #[arg(short, long)]
        status: bool,

        /// run git status
        #[arg(short('r'), long)]
        cmd: bool
    },
}

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
        Commands::Config { status: _ } => {
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