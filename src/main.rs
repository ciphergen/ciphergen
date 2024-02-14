mod generators;
mod command;

use command::{arguments::parse, execute::execute};
use log::LevelFilter::{Warn, Info, Debug};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = parse();
    let level = if arguments.verbosity.verbose { Debug } else if arguments.verbosity.quiet { Warn } else { Info };

    env_logger::builder().filter_level(level).init();

    execute(arguments)
}
