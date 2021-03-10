use anyhow::{Result};
use structopt::StructOpt;

use filminfo::input;
use filminfo::input::EntityType;
use filminfo::search;

/// Search for a film and display useful info for it
#[derive(StructOpt, Debug)] struct Cli {
    /// The entity type to get info for
    #[structopt(possible_values = &EntityType::variants(), case_insensitive = true)]
    entity_type: EntityType,
    /// search string
    search: String,
    /// path to config
    #[structopt(short, long, default_value="./config.json")]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    println!("entity_type: {}", &args.entity_type);
    println!("search: {}", &args.search);
    println!("config_path: {}", &args.config_path);

    let config = input::get_config(&args.config_path)?;
    println!("config: {:?}", &config);

    let results = search::make_search(
        &config, 
        &args.entity_type,
        &args.search
    ).await?;
    println!("Results found:");
    for result in results {
        println!("{:?}", result);
    }

    Ok(())
}
