use anyhow::Result;
mod dispatch;
pub use dispatch::Dispatch;
mod error;

mod add;
mod show;
mod find;
mod edit;
mod remove;
mod load;
mod save;
mod help;
mod status;

/// A command represents a piece of functionality that the user can invoke.
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub run: fn(&mut Dispatch, &[String]) -> Result<()>,
}

impl Command {
    /// Collects all commands registered via inventory.
    pub fn all() -> Vec<&'static Command> {
        inventory::iter::<Command>.into_iter().collect()
    }
}

inventory::collect!(Command);
