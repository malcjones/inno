use super::{Command, Dispatch};

pub fn run(dispatch: &mut Dispatch, args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Not enough arguments".to_string());
    }

    let name = &args[0];
    let url = &args[1];
    let tags = args.iter().skip(2).map(|s| s.to_string()).collect();

    let id = dispatch
        .store_mut()
        .create(name.to_string(), url.to_string(), tags);

    println!("Created bookmark with ID {}", id);

    Ok(())
}

inventory::submit!(Command {
    name: "add",
    description: "Add a new bookmark",
    usage: "add <name> <url> [tags...]",
    run,
});
