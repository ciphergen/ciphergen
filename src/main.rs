use rand::Rng;
use clap::{Parser, Subcommand, Args};
use log::LevelFilter::{Warn, Info, Debug};
use rand::distributions::{Alphanumeric, Uniform, Standard, DistString};
use rand::seq::IteratorRandom;

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
    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,

    #[arg(short, long, help = "Suppress informational messages")]
    quiet: bool
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new secret key
    Create {
        #[command(subcommand)]
        command: Option<CreateCommands>
    }
}

#[derive(Subcommand)]
enum CreateCommands {
    Binary {
        length: u16
    },
    Password {
        #[arg(short, long, help = "Use every available Unicode code point")]
        expanded: bool,
        length: u16
    },
    Passphrase {
        length: u16
    }
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    let hex_chars: Vec<String> = bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    hex_chars.join("")
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
        Some(Commands::Create { command }) => {
            match command {
                Some(CreateCommands::Binary { length }) => {
                    let range = Uniform::new(u8::MIN, u8::MAX);
                    let bytes: Vec<u8> = rand::thread_rng()
                        .sample_iter(range)
                        .take(*length as usize)
                        .collect();
                    let hex = to_hex_string(bytes);

                    println!("{}", hex);
                }
                Some(CreateCommands::Password { length, expanded }) => {
                    let password: String = if *expanded {
                        rand::thread_rng()
                            .sample_iter::<char, Standard>(Standard)
                            .take(*length as usize)
                            .map(char::from)
                            .collect()
                    }
                    else {
                        Alphanumeric.sample_string(&mut rand::thread_rng(), *length as usize)
                    };

                    println!("{}", password)
                }
                Some(CreateCommands::Passphrase { length }) => {
                    let wordlist = include_str!("wordlist.txt").to_string();
                    let passphrase = wordlist
                        .split('\n')
                        .map(|value| value.to_string())
                        .choose_multiple(&mut rand::thread_rng(), *length as usize)
                        .join(" ");

                    println!("{}", passphrase);
                }
                None => {}
            }
        }
        None => {}
    }
}
