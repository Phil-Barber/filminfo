use std::iter::Iterator;
use std::option::Option;
use dialoguer::Select;
use crate::search::SearchResult;

fn page<'a>(iter: &mut impl Iterator<Item = &'a SearchResult>, page_size: i8) -> Option<usize> {
    let mut selector = Select::new();
    for _ in 0..page_size {
        let next = iter.next();
        if !next.is_none() {
            selector.item(next.unwrap());
        }
    }
    selector.item("Show more");
    selector.interact().ok()
}

pub fn get_selected_item<'a>(
    mut iter: impl Iterator<Item = &'a SearchResult>
) -> Option<&'a SearchResult> {
    let page_size = 3;
    let chosen_index = page(iter.by_ref(), page_size)?;
    iter.nth(chosen_index)
}
