use std::io::{Write, stdout};
use super::arguments::{Arguments, Commands};
use super::generate::generate;

pub fn execute(arguments: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
    match &arguments.command {
        Commands::Generate { command } => {
            let bytes = generate(command)?;
            let mut stdout = stdout();

            stdout.write_all(&bytes)?;
            stdout.flush()?;

            Ok(())
        }
    }
}
