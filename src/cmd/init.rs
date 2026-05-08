use clap::{Parser, Subcommand};

#[derive(Parser)]
/// Simple program to greet a person
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
pub struct Cli {
    /// Enable verbose logging mode
    #[arg(short('v'), long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Context for sessions
    #[command(subcommand)]
    Context(GitSubcommands),

    Config {
        // /// Check config values
        // #[arg(short, long, default_value = "true")]
        // status: bool,
    },
}

#[derive(Subcommand)]
pub enum GitSubcommands {
    /// Manages Context for Git
    Git {
        /// Create storage file for gitshit later :3
        #[arg(long)]
        create: bool,

        /// Collect local git repo dir to do stuff to it later :3
        #[arg(short, long)]
        collect: Option<String>,

        /// run git status
        #[arg(short, long)]
        status: bool,

        /// run git status
        #[arg(short('r'), long)]
        cmd: bool
    },
}