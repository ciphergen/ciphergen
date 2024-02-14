use std::io::{stdin, stdout, Read, Write};

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
            let mut stdin = stdin();
            let mut buffer = Vec::<u8>::new();

            match input {
                Some(value) => {
                    buffer = value.as_bytes().to_vec();
                },
                None => {
                    stdin.read_to_end(&mut buffer)?;
                }
            };

            println!("{}", analyze(buffer));

            Ok(())
        }
    }
}
