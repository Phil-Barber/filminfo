use anyhow::{Result};
use structopt::StructOpt;
use kuchiki::traits::*;

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

    let query_string = search::build_query(&args.entity_type, &args.search);
    let url = format!(
        "{base_url}{query_string}", 
        base_url = &config.base_url,
        query_string = &query_string,
    );
    let res = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let dom = kuchiki::parse_html().one(res);

    let result = search::chose_result(&dom);

    Ok(())
}
