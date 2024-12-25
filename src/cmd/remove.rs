use super::error::InvalidArguments;
use super::{Command, Dispatch};
use anyhow::{Context, Result};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [id] => {
            let id = id.parse().context("Invalid ID")?;
            dispatch
                .store_mut()
                .remove(id)
                .context("Bookmark not found")?;
            println!("Removed bookmark with ID {}", id);
        }
        _ => anyhow::bail!(InvalidArguments),
    }

    Ok(())
}

inventory::submit!(Command {
    name: "remove",
    description: "Remove a bookmark",
    usage: "remove <id>",
    run,
});
