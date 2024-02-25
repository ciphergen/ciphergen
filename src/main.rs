mod generators;
mod command;
mod wordlist;

use std::io::Write;

use command::{arguments::parse, execute::execute};
use log::{LevelFilter::{Warn, Info, Trace, Error}, error};

fn main() {
    let arguments = parse();
    let mut builder = env_logger::builder();

    if arguments.verbosity.debug { builder.filter_level(Trace); }
    else if arguments.verbosity.verbose { builder.filter_level(Info); }
    else if arguments.verbosity.quiet { builder.filter_level(Error); }
    else { builder.filter_level(Warn); };

    builder.format(|buffer, record| writeln!(buffer, "{}", record.args()));
    builder.init();

    if let Some(error) = execute(arguments).err() {
        error!("Error: {}", error);
    };
}
