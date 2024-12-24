pub mod cmd;
pub mod store;

use cmd::{Command, Dispatch};

fn main() -> Result<(), String> {
    Dispatch::default().with_commands(Command::all()).start()
}
