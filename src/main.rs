use anyhow::Result;
use structopt::StructOpt;
use clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    enum EntityType {
        Film,
        Actor,
    }
}

/// Search for a film and display useful info for it
#[derive(StructOpt, Debug)] struct Cli {
    /// The entity type to get info for
    #[structopt(possible_values = &EntityType::variants(), case_insensitive = true)]
    entity_type: EntityType,
    /// search string
    search: String,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    println!("entity_type: {}", &args.entity_type);
    println!("search: {}", &args.search);
    Ok(())
}
