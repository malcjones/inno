use anyhow::{Context, Result};
use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [id, new_name, new_url, new_tags @ ..] => {
            let id = id.parse().context("Invalid ID")?;
            let mut bookmark = dispatch.store_mut().get_mut(id).context("Bookmark not found")?;
            bookmark.name.clone_from(new_name);
            bookmark.url.clone_from(new_url);
            bookmark.tags = new_tags.iter().cloned().collect();
            println!("Edited bookmark with ID {}", id);
        }
        _ => anyhow::bail!("Invalid arguments"),
    }

    Ok(())
}

inventory::submit!(Command {
    name: "edit",
    description: "Edit an existing bookmark",
    usage: "edit <id> <new_name> [new_url] [new_tags...]",
    run,
});
