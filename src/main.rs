use std::fmt;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, enum_utils::FromStr)]
enum EntityType {
    #[enumeration(rename="film")]
    Film,
    #[enumeration(rename="actor")]
    Actor,
}
impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Film => write!(f, "film"),
            Self::Actor => write!(f, "actor"),
        }
    }
}

/// Search for a film and display useful info for it
#[derive(StructOpt)] struct Cli {
    /// The entity type to get info for
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
