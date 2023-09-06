use clap::{Parser, Subcommand, Args};

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
    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    pub verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    pub quiet: bool
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Radix {
    #[arg(short = 'H', long = "hex", help = "Encode output as base16")]
    pub hex: bool,

    #[arg(short = 'B', long = "base64", help = "Encode output as base64")]
    pub base64: bool
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new secret key or username
    Generate {
        #[command(subcommand)]
        command: GenerateCommands
    }
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Generate random bytes
    Bytes {
        /// The number of bytes to generate
        count: u64
    },
    /// Generate random bytes and encode them as a hexadecimal string
    Hex {
        #[arg(short = 'u', long = "uppercase", help = "Print hexadecimal digits in uppercase")]
        uppercase: bool,

        /// The number of bytes to generate
        count: u64
    },
    /// Generate random bytes and encode them as a Base64 string
    Base64 {
        #[arg(short = 's', long = "url-safe", help = "Use a URL-safe alphabet")]
        url_safe: bool,

        /// The number of bytes to generate
        count: u64
    },
    /// Generate a random password
    Password {
        #[command(subcommand)]
        command: PasswordCommands
    },
    /// Generate a random passphrase
    Passphrase {
        #[arg(short = 'd', long = "delimiter", help = "The substring used to separate words from each other")]
        delimiter: Option<String>,

        #[arg(short = 'p', long = "path", help = "the wordlist file to read into memory")]
        path: Option<String>,

        /// The number of words to generate
        length: u64
    },
    /// Generate a random username
    Username {
        #[command(subcommand)]
        command: UsernameCommands
    },
    /// Generate random digits
    Digits {
        /// The number of digits to generate
        count: u64
    },
    /// Generate a random number
    Number {
        /// The smallest number that can be generated
        minimum: u64,

        /// The largest number that can be generated
        maximum: u64
    }
}

#[derive(Subcommand)]
pub enum UsernameCommands {
    /// Create a simple username that alternates between vowels and consonants
    Simple {
        /// The number of characters to generate
        length: u64
    },
    /// Create a complex username that is constructed from syllables
    Syllabic {
        /// The number of syllables to generate
        length: u64
    }
}

#[derive(Subcommand)]
pub enum PasswordCommands {
    /// Create a password that consists of random ASCII code points
    Alphabetic {
        /// The number of characters to generate
        length: u64
    },
    /// Create a password that consists of random Unicode code points
    Full {
        /// The number of characters to generate
        length: u64
    }
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
