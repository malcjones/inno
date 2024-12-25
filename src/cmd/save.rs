use anyhow::Result;
use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [path] => dispatch.store().save(path),
        _ => anyhow::bail!("Invalid arguments"),
    }
}

inventory::submit!(Command {
    name: "save",
    description: "Save bookmarks to a file",
    usage: "save <path>",
    run,
});