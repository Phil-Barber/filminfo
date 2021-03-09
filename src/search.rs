use anyhow::{Result};
use kuchiki::{ElementData, NodeDataRef, NodeRef};
use crate::input::EntityType;

#[derive(Debug)] pub struct SearchResult {
    url: String,
    display: String,
}
impl SearchResult {
    pub fn new(url: String, display: String) -> Self {
        Self {url, display}
    }

    pub fn from_node_ref(node_ref: NodeDataRef<ElementData>) -> Self {
        let node = node_ref.as_node();
        let text = node.text_contents();
        let display = text.trim();

        let anchor = node
            .select("a")
            .unwrap()
            .next()
            .unwrap();
        let element = anchor
            .as_node()
            .as_element()
            .unwrap();

        let attributes = element.attributes.borrow();
        let url = attributes.get("href").unwrap();
        Self::new(String::from(display), String::from(url))
    }
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
        let result = SearchResult::from_node_ref(dom_match);
        println!("{:?}", result);
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
