use anyhow::Result;
use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<()> {
    match args {
        [] => {
            println!("Available commands:");
            for command in dispatch.commands() {
                println!("  {:<10} {}", command.name, command.description);
            }
        }
        [name] => {
            if let Some(command) = dispatch.command(name) {
                println!("Usage: {} {}", command.name, command.usage);
            } else {
                eprintln!("Unknown command: {}", name);
            }
        }
        _ => anyhow::bail!("Invalid arguments"),
    }

    Ok(())
}

inventory::submit!(Command {
    name: "help",
    description: "Show help for a command or list all commands",
    usage: "help [command]",
    run,
});
