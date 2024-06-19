use std::path::PathBuf;

use clap::{Parser, Subcommand, Args, ArgAction};
use log::LevelFilter;

type UnitResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: Verbosity
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Verbosity {
    #[arg(short = 'd', long = "debug", help = "Enable debugging output", global = true)]
    pub debug: bool,

    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    pub verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    pub quiet: bool
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new secret key or username
    Generate {
        /// The command to execute
        #[command(subcommand)]
        command: GenerateCommands
    },
    /// Cryptographically analyze a piece of data
    Analyze {
        /// A path to a file on a filesystem, or leave empty to read from STDIN
        input: Option<PathBuf>
    }
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Generate random bytes
    Bytes {
        /// The number of bytes to generate
        length: usize
    },
    /// Generate random bytes and encode them as a hexadecimal string
    Hex {
        #[arg(short = 'u', long = "uppercase", help = "Print hexadecimal digits in uppercase")]
        uppercase: bool,

        /// The number of bytes to generate
        length: usize
    },
    /// Generate random bytes and encode them as a Base64 string
    Base64 {
        #[arg(short = 's', long = "url-safe", help = "Use a URL-safe alphabet")]
        url_safe: bool,

        /// The number of bytes to generate
        length: usize
    },
    /// Generate a random password with a configurable character set
    Password {
        #[arg(short = 'n', long = "numbers", help = "Don't include any digits", action = ArgAction::SetFalse)]
        numbers: bool,

        #[arg(short = 's', long = "symbols", help = "Don't include any symbols", action = ArgAction::SetFalse)]
        symbols: bool,

        /// The number of characters to generate
        length: usize,

        /// How many passwords to generate
        count: Option<usize>
    },
    /// Generate a passphrase composed of words chosen at random from a wordlist
    Passphrase {
        #[arg(short = 'p', long = "path", help = "The wordlist file to read into memory")]
        path: Option<PathBuf>,

        #[arg(short = 'D', long = "delimiter", help = "The string used to separate words from each other in the wordlist", default_value = "\n")]
        delimiter: String,

        #[arg(short = 's', long = "separator", help = "A string used to separate words in the passphrase", default_value = " ")]
        separator: String,

        /// The number of words to generate
        length: usize,

        /// How many passphrases to generate
        count: Option<usize>
    },
    /// Generate a random pronounceable username
    Username {
        #[arg(short = 'C', long = "capitalize", help = "Make the first letter uppercase", global = true)]
        capitalize: bool,

        #[command(subcommand)]
        command: UsernameCommands
    },
    /// Generate a random sequence of digits
    Digits {
        /// The number of digits to generate
        length: usize,

        /// How many sequences of digits to generate
        count: Option<usize>
    },
    /// Generate a random number
    Number {
        /// The smallest number that can be generated
        minimum: usize,

        /// The largest number that can be generated
        maximum: usize,

        /// How many numbers to generate
        count: Option<usize>
    }
}

#[derive(Subcommand)]
pub enum UsernameCommands {
    /// Generate a simple pronounceable username that alternates between vowels and consonants
    Simple {
        /// The number of characters to generate
        length: usize,

        /// How many simple usernames to generate
        count: Option<usize>
    },
    /// Generate a complex pronounceable username that is constructed from syllables
    Complex {
        /// The number of syllables to generate
        length: usize,

        /// How many syllabic usernames to generate
        count: Option<usize>
    }
}

impl Verbosity {
    fn to_filter(&self) -> LevelFilter {
        if self.debug { LevelFilter::Trace }
        else if self.verbose { LevelFilter::Debug }
        else if self.quiet { LevelFilter::Warn }
        else { LevelFilter::Info }
    }
}

pub fn parse() -> Arguments {
    Arguments::parse()
}

pub fn setup_logging(verbosity: &Verbosity) -> UnitResult {
    let filter = verbosity.to_filter();

    env_logger::builder()
        .filter_level(filter)
        .format_level(true)
        .format_target(false)
        .format_module_path(false)
        .format_timestamp_secs()
        .parse_default_env()
        .try_init()?;

    Ok(())
}
