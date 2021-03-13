use std::fmt;
use anyhow::{Result};
use kuchiki::{ElementData, NodeDataRef, NodeRef};
use kuchiki::traits::*;
use crate::input::{Config, EntityType};

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
        Self::new(String::from(url), String::from(display))
    }
}
impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

pub async fn make_search(
    config: &Config, 
    entity_type: &EntityType, 
    search: &str
) -> Result<Vec<SearchResult>> {
    let query_string = build_query(&entity_type, &search);
    let url = format!(
        "{base_url}find?{query_string}", 
        base_url = &config.base_url,
        query_string = &query_string,
    );
    let res = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let dom = kuchiki::parse_html().one(res);
    return get_results_from_dom(&dom);
}

fn build_query(entity_type: &EntityType, search: &str) -> String {
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

fn get_results_from_dom(dom: &NodeRef) -> Result<Vec<SearchResult>> {
    let dom_results = dom.select(".findResult").unwrap();
    let search_results = dom_results
        .map(|dom_match| SearchResult::from_node_ref(dom_match))
        .collect();
    Ok(search_results)
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


