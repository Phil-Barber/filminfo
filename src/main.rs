use anyhow::{Result};
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

    let config = filminfo::get_config(&args.config_path)?;
    println!("config: {:?}", &config);

    let result = reqwest::get(&config.base_url)
        .await?;
    println!("{:?}", result);

    Ok(())
}
