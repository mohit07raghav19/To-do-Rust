use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the TODO file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the TODO File by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the TODO File
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rust TODO", about = "A CLI based TODO in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different TODO File.
    #[structopt(parse(from_os_str), short, long)]
    pub todo_file: Option<PathBuf>,
}
