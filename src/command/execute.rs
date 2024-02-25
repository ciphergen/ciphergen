use std::fs::read;
use std::io::{stdin, stdout, Read, Write};

use log::error;

use super::analyze::analyze;
use super::arguments::{Arguments, Commands};
use super::generate::generate;

pub fn execute(arguments: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    match arguments.command {
        Commands::Generate { command } => {
            let bytes = generate(command)?;
            let mut stdout = stdout();

            stdout.write_all(&bytes)?;
            stdout.flush()?;

            Ok(())
        }
        Commands::Analyze { input } => {
            let mut buffer = Vec::<u8>::new();

            match input {
                Some(value) => {
                    buffer = read(value)?;
                },
                None => {
                    stdin().read_to_end(&mut buffer)?;
                }
            };

            if buffer.is_empty() { error!("There is no data to read") }

            println!("{}", analyze(buffer));

            Ok(())
        }
    }
}
