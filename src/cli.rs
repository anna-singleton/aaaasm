use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="Anna Singleton")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run an AAAASM program and output the accumulator at the end
    Run {
        /// The .aaaasm file to load the instructions from
        #[arg(required=true)]
        file: String,

        /// Print accumulator value after each instruction.
        #[arg(short, long)]
        trace: bool
    },
}
