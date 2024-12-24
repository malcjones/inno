use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<(), String> {
    match args.len() {
        0 => {
            for command in dispatch.commands() {
                println!("{} - {}", command.name, command.description);
            }
        }
        1 => {
            let name = &args[0];
            let command = dispatch.commands().iter().find(|c| c.name == name);

            println!("Usage: {}", command.map_or("Unknown command", |c| c.usage));
        }
        _ => return Err("Too many arguments".to_string()),
    }

    Ok(())
}

inventory::submit!(Command {
    name: "help",
    description: "Show help for a command or list all commands",
    usage: "help [command]",
    run,
});
