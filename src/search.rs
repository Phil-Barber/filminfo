use anyhow::{Result};
use kuchiki::NodeRef;
use crate::input::EntityType;

#[derive(Debug)] pub struct SearchResult {
    url: String,
    display: String,
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
        println!("Up: {}", &dom_match.as_node().text_contents());
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
        println!("{:?}", child.text_contents())
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
