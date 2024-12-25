use anyhow::Result;

use super::{Command, Dispatch};

use itertools::Itertools;

pub fn run(dispatch: &mut Dispatch, _: &[String]) -> Result<()> {
    println!("inno v{}", env!("CARGO_PKG_VERSION"));

    println!(
        "entries: {}; bookmarks: {}",
        dispatch.store().entry_count(),
        dispatch.store().bookmark_count()
    );

    let tags = &dispatch.store().tag_index;

    if tags.is_empty() {
        println!("no tags");
    } else {
        println!(
            "tags: {}",
            tags.iter()
                .sorted_by_key(|(_, ids)| -(ids.len() as i64))
                .map(|(tag, _)| tag)
                .join(", ")
        );
    }

    Ok(())
}

inventory::submit!(Command {
    name: "status",
    description: "Show status",
    usage: "status",
    run,
});
