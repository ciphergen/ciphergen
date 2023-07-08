mod generators;

use std::io::Write;

use clap::{Parser, Subcommand, Args};
use log::LevelFilter::{Warn, Info, Debug};
use generators::{
    generate_binary,
    generate_hexadecimal,
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
        #[arg(short = 'H', long = "hexadecimal", help = "Output binary data in hexadecimal format")]
        hexadecimal: bool,
        length: u16
    },
    /// Generate a random password
    Password {
        #[arg(short = 'e', long = "expanded", help = "Use every available Unicode code point")]
        expanded: bool,
        length: u16
    },
    /// Generate a random passphrase
    Passphrase {
        length: u16
    },
    /// Generate a random username
    Username {
        length: u16
    },
    /// Generate a random PIN
    Pin {
        length: u16
    }
}

fn main() {
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
                Some(GenerateCommands::Binary { hexadecimal, length }) => {
                    if *hexadecimal {
                        let hex = generate_hexadecimal(length);

                        println!("{}", hex);
                    }
                    else {
                        let bytes = generate_binary(length);
                        let mut stdout = std::io::stdout();

                        stdout.write_all(&bytes).unwrap();
                        stdout.flush().unwrap();
                    }
                }
                Some(GenerateCommands::Password { expanded, length }) => {
                    let password = generate_password(expanded, length);

                    println!("{}", password);
                }
                Some(GenerateCommands::Passphrase { length }) => {
                    let passphrase = generate_passphrase(length);

                    println!("{}", passphrase);
                }
                Some(GenerateCommands::Username { length }) => {
                    let username = generate_username(length);

                    println!("{}", username);
                }
                Some(GenerateCommands::Pin { length }) => {
                    let pin = generate_pin(length);

                    println!("{}", pin);
                }
                None => {}
            }
        }
        None => {}
    }
}
