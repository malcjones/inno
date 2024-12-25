use anyhow::{Result, Context};
use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [id] => {
            let id = id.parse().context("Invalid ID")?;
            let bookmark = dispatch.store().get(id).context("Bookmark not found")?;
            println!("{}", bookmark);
        }
        _ => {
            for bookmark in dispatch.store().iter() {
                println!("{}", bookmark);
            }
        }
    }

    Ok(())
}

inventory::submit!(Command {
    name: "show",
    description: "Show bookmarks",
    usage: "show [id]",
    run,
});