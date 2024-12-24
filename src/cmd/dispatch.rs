use rustyline::{DefaultEditor, error::ReadlineError};

use super::Command;
use crate::store::Store;

pub struct Dispatch {
    store: Store,
    commands: Vec<&'static Command>,
    editor: DefaultEditor,
}

impl Dispatch {
    /// Create a new dispatch
    pub fn new(store: Store, commands: Vec<&'static Command>) -> Self {
        Self {
            store,
            commands,
            editor: DefaultEditor::new().expect("Failed to create editor"),
        }
    }

    /// Get the commands for the dispatch
    pub fn commands(&self) -> &[&'static Command] {
        &self.commands
    }

    /// Set the commands for the dispatch
    pub fn with_commands(mut self, commands: Vec<&'static Command>) -> Self {
        self.commands = commands;
        self
    }

    /// Get a reference to the store
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Get a mutable reference to the store
    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub fn run(&mut self, command_name: &str, args: &[String]) -> Result<(), String> {
        match self.commands.iter().find(|c| c.name == command_name) {
            Some(command) => (command.run)(self, args),
            None => Err(format!("Command '{}' not found", command_name)),
        }
    }

    /// Read a line from the user
    pub fn take_line(&mut self) -> Result<String, ReadlineError> {
        static PROMPT: &str = ">>> ";

        // Read a line from the user
        let line = self.editor.readline(PROMPT)?;

        // Add the line to the history
        self.editor.add_history_entry(&line)?;

        Ok(line)
    }

    pub fn parse_args(line: &str) -> Result<(Option<String>, Vec<String>), String> {
        let (mut quoted, mut escaped) = (false, false);
        let mut current = String::new();
        let mut args = Vec::new();

        for c in line.chars() {
            match c {
                '\\' if !escaped => escaped = true,
                '"' if !escaped => {
                    quoted = {
                        if quoted {
                            args.push(current);
                            current = String::new();
                        }
                        !quoted
                    }
                }
                ' ' if !quoted && !escaped => {
                    if !current.is_empty() {
                        args.push(current);
                        current = String::new();
                    }
                }
                _ => {
                    current.push(c);
                    escaped = false;
                }
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        if quoted {
            return Err("Unterminated quote".to_string());
        }

        Ok((
            args.first().map(|s| s.to_string()),
            args.into_iter().skip(1).collect(),
        ))
    }

    pub fn run_line(&mut self, line: &str) -> Result<(), String> {
        let (command, args) = Self::parse_args(line)?;

        if let Some(command) = command {
            self.run(&command, args.as_slice())
        } else {
            Ok(())
        }
    }

    /// Start the dispatch loop
    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let line = match self.take_line() {
                Ok(line) => line,
                Err(ReadlineError::Eof) => break Ok(()),
                Err(ReadlineError::Interrupted) => continue,
                Err(e) => return Err(e.to_string()),
            };

            if line.is_empty() {
                continue;
            }

            if let Err(e) = self.run_line(&line) {
                eprintln!("{}", e);
            }
        }
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::new(Store::default(), vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_simple() {
        let line = "add Rust \"https://www.rust-lang.org\" rust,lang";
        let (command, args) = Dispatch::parse_args(line).expect("Failed to parse");
        assert_eq!(command.unwrap(), "add");
        assert_eq!(args, vec!["Rust", "https://www.rust-lang.org", "rust,lang"]);
    }

    #[test]
    fn test_parse_args_quote_no_whitespace() {
        let line = "add \"Rust\"https://www.rust-lang.org";
        let (command, args) = Dispatch::parse_args(line).expect("Failed to parse");
        assert_eq!(command.unwrap(), "add");
        assert_eq!(args, vec!["Rust", "https://www.rust-lang.org"]);
    }

    #[test]
    fn test_parse_args_escaped_quotes() {
        let line = "add \"Hello \\\"World\\\"\" https://example.com";
        let (command, args) = Dispatch::parse_args(line).expect("Failed to parse");
        assert_eq!(command.unwrap(), "add");
        assert_eq!(args, vec!["Hello \"World\"", "https://example.com"]);
    }

    #[test]
    fn test_unterminated_quote() {
        let line = "add \"Bad argument";
        assert!(Dispatch::parse_args(line).is_err());
    }
}
