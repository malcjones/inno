use anyhow::Result;
mod dispatch;
pub use dispatch::Dispatch;
mod error;

mod add;
mod edit;
mod find;
mod help;
mod load;
mod remove;
mod save;
mod show;
mod status;

/// Represents a user-invoked command within the application.
/// Each command includes a name, description, usage instructions, and a run function.
pub struct Command {
    /// The name of the command (e.g., "add", "edit").
    pub name: &'static str,
    /// A short description of what the command does.
    pub description: &'static str,
    /// Usage instructions for the command (e.g., "add <item>").
    pub usage: &'static str,
    /// The function to execute when the command is invoked.
    pub run: fn(&mut Dispatch, &[String]) -> Result<()>,
}

impl Command {
    /// Collects all commands registered via inventory.
    ///
    /// # Returns
    ///
    /// A vector of references to all registered commands.
    pub fn all() -> Vec<&'static Command> {
        inventory::iter.into_iter().collect()
    }
}

inventory::collect!(Command);
