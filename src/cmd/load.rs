use super::{Command, Dispatch};
use anyhow::Result;

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [path] => dispatch.store_mut().load(path),
        _ => anyhow::bail!("Invalid arguments"),
    }
}

inventory::submit!(Command {
    name: "load",
    description: "Load bookmarks from a file",
    usage: "load <path>",
    run,
});
