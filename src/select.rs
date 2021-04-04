use std::iter::Iterator;
use std::option::Option;
use dialoguer::Select;
use crate::search::SearchResult;

fn page<'a>(
    iter: &mut impl Iterator<Item = &'a SearchResult>, 
    page_size: usize,
) -> Option<usize> {
    let mut selector = Select::new();
    for i in 0..page_size {
        let next = iter.next();
        if !next.is_none() {
            selector.item(next.unwrap());
        } else if i == 0 {
            panic!("No more options")
        }
    }
    selector.item("Show more");
    let chosen = selector.interact().ok()?;
    if chosen == page_size {
        return Some(page_size + page(iter, page_size)?);
    }
    Some(chosen)
}

pub fn get_selected_item<'a>(
    mut iter: impl Iterator<Item = &'a SearchResult>
) -> Option<&'a SearchResult> {
    let page_size = 3;
    let chosen_index = page(iter.by_ref(), page_size)?;
    iter.nth(chosen_index)
}
