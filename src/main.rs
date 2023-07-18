mod generators;

use std::{io::Write, error::Error};

use clap::{Parser, Subcommand, Args};
use log::LevelFilter::{Warn, Info, Debug};
use generators::{
    SecretKeyLength,
    generate_binary,
    generate_hexadecimal,
    generate_base64,
    generate_password,
    generate_passphrase,
    generate_simple_username,
    generate_syllabic_username,
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
    /// Generate random bytes
    Binary {
        /// The number of bytes to generate
        length: SecretKeyLength
    },
    /// Generate random bytes and encode them as a hexadecimal string
    Hexadecimal {
        /// The number of bytes to generate
        length: SecretKeyLength
    },
    /// Generate random bytes and encode them as a Base64 string
    Base64 {
        /// The number of bytes to generate
        length: SecretKeyLength
    },
    /// Generate a random password
    Password {
        #[arg(short = 'e', long = "expanded", help = "Use every available Unicode code point")]
        expanded: bool,

        /// The number of characters to generate
        length: SecretKeyLength
    },
    /// Generate a random passphrase
    Passphrase {
        #[arg(short = 'd', long = "delimiter", help = "The substring used to separate words from each other")]
        delimiter: Option<String>,

        #[arg(short = 'p', long = "path", help = "the wordlist file to read into memory")]
        path: Option<String>,

        /// The number of words to generate
        length: SecretKeyLength
    },
    /// Generate a random username
    Username {
        #[command(subcommand)]
        command: UsernameCommands
    },
    /// Generate a random PIN
    Pin {
        /// The number of digits to generate
        length: SecretKeyLength
    }
}

#[derive(Subcommand)]
enum UsernameCommands {
    /// Create a simple username that alternates between vowels and consonants
    Simple {
        /// The number of characters to generate
        length: SecretKeyLength
    },
    /// Create a complex username that is constructed from syllables
    Syllabic {
        /// The number of syllables to generate
        length: SecretKeyLength
    }
}

fn generate(subcommand: &Option<GenerateCommands>) -> Result<(), Box<dyn Error>> {
    match subcommand {
        Some(GenerateCommands::Binary { length }) => {
            let bytes = generate_binary(length)?;
            let mut stdout = std::io::stdout();

            stdout.write_all(&bytes).unwrap();
            stdout.flush().unwrap();

            Ok(())
        }
        Some(GenerateCommands::Hexadecimal { length }) => {
            let hexadecimal = generate_hexadecimal(length)?;

            println!("{}", hexadecimal);

            Ok(())
        }
        Some(GenerateCommands::Base64 { length }) => {
            let base64 = generate_base64(length)?;

            println!("{}", base64);

            Ok(())
        }
        Some(GenerateCommands::Password { expanded, length }) => {
            let password = generate_password(expanded, length)?;

            println!("{}", password);

            Ok(())
        }
        Some(
            GenerateCommands::Passphrase {
                length,
                delimiter,
                path
            }) => {
                let passphrase = generate_passphrase(length, delimiter, path)?;

                println!("{}", passphrase);

                Ok(())
            }
        Some(GenerateCommands::Username { command }) => {
            match command.into() {
                Some(UsernameCommands::Simple { length }) => {
                    let username = generate_simple_username(length)?;

                    println!("{}", username);

                    Ok(())
                }
                Some(UsernameCommands::Syllabic { length }) => {
                    let username = generate_syllabic_username(length)?;

                    println!("{}", username);

                    Ok(())
                }
                None => panic!()

            }
        }
        Some(GenerateCommands::Pin { length }) => {
            let pin = generate_pin(length)?;

            println!("{}", pin);

            Ok(())
        }
        None => panic!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
            generate(command)?;

            Ok(())
        },
        None => panic!()
    }
}
