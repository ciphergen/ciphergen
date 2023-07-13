mod generators;

use std::io::Write;

use clap::{Parser, Subcommand, Args};
use log::LevelFilter::{Warn, Info, Debug};
use generators::{
    SecretKeyLength,
    generate_binary,
    generate_hexadecimal,
    generate_base64,
    generate_password,
    generate_passphrase,
    generate_username,
    generate_pin
};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbosity: Verbosity
}

#[derive(Args)]
#[group(multiple = false)]
struct Verbosity {
    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    quiet: bool
}

#[derive(Args)]
#[group(multiple = false)]
struct Radix {
    #[arg(short = 'H', long = "hex", help = "Encode output as base16")]
    hex: bool,

    #[arg(short = 'B', long = "base64", help = "Encode output as base64")]
    base64: bool
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new secret key or username
    Generate {
        #[command(subcommand)]
        command: Option<GenerateCommands>
    }
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate a random sequence of bytes
    Binary {
        #[command(flatten)]
        radix: Radix,

        length: SecretKeyLength
    },
    /// Generate a random password
    Password {
        #[arg(short = 'e', long = "expanded", help = "Use every available Unicode code point")]
        expanded: bool,

        length: SecretKeyLength
    },
    /// Generate a random passphrase
    Passphrase {
        length: SecretKeyLength
    },
    /// Generate a random username
    Username {
        length: SecretKeyLength
    },
    /// Generate a random PIN
    Pin {
        length: SecretKeyLength
    }
}

fn main() -> Result<(), String> {
    let arguments = Arguments::parse();
    let mut builder = env_logger::builder();

    if arguments.verbosity.verbose {
        builder.filter_level(Debug);
    }
    else if arguments.verbosity.quiet {
        builder.filter_level(Warn);
    }
    else {
        builder.filter_level(Info);
    }

    builder.init();

    match &arguments.command {
        Some(Commands::Generate { command }) => {
            match command {
                Some(GenerateCommands::Binary { radix, length }) => {
                    if radix.hex {
                        match generate_hexadecimal(length) {
                            Ok(value) => {
                                println!("{}", value);
                                Ok(())
                            }
                            Err(message) => {
                                Err(message)
                            }
                        }
                    }
                    else if radix.base64 {
                        match generate_base64(length) {
                            Ok(value) => {
                                println!("{}", value);
                                Ok(())
                            }
                            Err(message) => {
                                Err(message)
                            }
                        }
                    }
                    else {
                        match generate_binary(length) {
                            Ok(value) => {
                                let mut stdout = std::io::stdout();

                                stdout.write_all(&value).unwrap();
                                stdout.flush().unwrap();

                                Ok(())
                            }
                            Err(message) => {
                                Err(message)
                            }
                        }
                    }
                }
                Some(GenerateCommands::Password { expanded, length }) => {
                    match generate_password(expanded, length) {
                        Ok(value) => {
                            println!("{}", value);
                            Ok(())
                        }
                        Err(message) => {
                            Err(message)
                        }
                    }
                }
                Some(GenerateCommands::Passphrase { length }) => {
                    match generate_passphrase(length) {
                        Ok(value) => {
                            println!("{}", value);
                            Ok(())
                        }
                        Err(message) => {
                            Err(message)
                        }
                    }
                }
                Some(GenerateCommands::Username { length }) => {
                    match generate_username(length) {
                        Ok(value) => {
                            println!("{}", value);
                            Ok(())
                        }
                        Err(message) => {
                            Err(message)
                        }
                    }
                }
                Some(GenerateCommands::Pin { length }) => {
                    match generate_pin(length) {
                        Ok(value) => {
                            println!("{}", value);
                            Ok(())
                        }
                        Err(message) => {
                            Err(message)
                        }
                    }
                }
                None => panic!()
            }
        }
        None => panic!()
    }
}
