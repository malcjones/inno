pub mod dispatch;
pub use dispatch::Dispatch;

pub mod add;
pub mod help;
pub mod list;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub run: fn(&mut Dispatch, &[String]) -> Result<(), String>,
}

impl Command {
    pub fn all() -> Vec<&'static Command> {
        inventory::iter::<Command>.into_iter().collect()
    }
}

inventory::collect!(Command);
