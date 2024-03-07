use std::fs::read;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread::spawn;

use log::error;

use super::analyze::analyze;
use super::arguments::{Arguments, Commands};
use super::generate::generate;

pub fn execute(arguments: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    match arguments.command {
        Commands::Generate { command } => {
            let (sender, receiver) = channel::<Vec<u8>>();
            let mut stdout = stdout();

            let handle = spawn(move || {
                generate(sender, command)
            });

            for message in receiver {
                stdout.write_all(&message)?;
            }

            stdout.flush()?;

            handle.join().unwrap();

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

            if buffer.is_empty() {
                error!("There is no data to read");
                exit(-1);
             }

            println!("{}", analyze(buffer));

            Ok(())
        }
    }
}
