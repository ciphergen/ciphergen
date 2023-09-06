use super::arguments::Commands;
use super::generate::generate;

pub fn execute(command: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Generate { command } => {
            generate(command)?;

            Ok(())
        }
    }
}
