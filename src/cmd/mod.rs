
pub mod dispatch;
pub use dispatch::Dispatch;

pub mod add;
pub mod help;
pub mod list;

/// A command represents a piece of functionality that the user can invoke.
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub run: fn(&mut Dispatch, &[String]) -> Result<(), String>,
}

impl Command {
    /// Collects all commands registered via inventory.
    pub fn all() -> Vec<&'static Command> {
        inventory::iter::<Command>.into_iter().collect()
    }
}

inventory::collect!(Command);
