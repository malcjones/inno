use anyhow::Result;
use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [query] => {
            let bookmarks = dispatch.store().find(query);
            for bookmark in bookmarks {
                println!("{}", bookmark);
            }
        }
        _ => anyhow::bail!("Invalid arguments"),
    }

    Ok(())
}

inventory::submit!(Command {
    name: "find",
    description: "Find bookmarks by name or tag",
    usage: "find <query>",
    run,
});