use clap::arg_enum;
use anyhow::{Context, Result};
use kuchiki::NodeRef;

arg_enum! {
    #[derive(Debug)] pub enum EntityType {
        Film,
        Actor,
    }
}

#[derive(Debug, serde::Deserialize)] pub struct Config {
    pub base_url: String,
}

pub fn get_config(
    config_path: &String
) -> Result<Config> {
    let contents = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Could not read file: {}", &config_path))?;

    let config: Config = serde_json::from_str(&contents)
        .with_context(|| format!("Invalid JSON:\n {}", &contents))?;
    Ok(config)
}


pub fn build_query(entity_type: &EntityType, search: &str) -> String {
    let e_type_str = match entity_type {
        EntityType::Film => "tt",
        EntityType::Actor => "nm",
    };
    format!(
        "s={e_type_str}&q={search}",
        e_type_str=e_type_str,
        search=search
    )
}

pub fn chose_result(dom: &NodeRef) -> Result<()> {
    let results = dom.select(".findResult").unwrap();
    for dom_match in results.take(3) {
        unpack_result(&dom_match.as_node())?;
    }
    Ok(())
}

fn unpack_result(result: &NodeRef) -> Result<()> {
    let result_children = result
        .select(".result_text")
        .unwrap()
        .next()
        .unwrap()
        .as_node()
        .children();
    println!("Printing child:");
    for child in result_children {
        println!("{:?}", child)
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_string() {
        let search = "test";
        assert_eq!(
            build_query(&EntityType::Film, &search),
            "s=tt&q=test",
        );
        assert_eq!(
            build_query(&EntityType::Actor, &search),
            "s=nm&q=test",
        );
    }
}
