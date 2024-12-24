use std::io::Write;

use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, _: &[String]) -> Result<(), String> {
    let mut stdout = std::io::stdout();

    for bookmark in dispatch.store().iter() {
        writeln!(stdout, "{}", bookmark).map_err(|e| e.to_string())?;
    }

    Ok(())
}

inventory::submit!(Command {
    name: "list",
    description: "List bookmarks",
    usage: "list",
    run,
});
