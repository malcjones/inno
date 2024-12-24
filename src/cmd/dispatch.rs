use rustyline::{DefaultEditor, error::ReadlineError};

use super::Command;
use crate::store::Store;

/// A dispatcher for handling commands and managing state.
pub struct Dispatch {
    store: Store,
    commands: Vec<&'static Command>,
    editor: DefaultEditor,
}

impl Dispatch {
    /// Creates a new `Dispatch`.
    ///
    /// # Arguments
    ///
    /// * `store` - The store used to manage state.
    /// * `commands` - A list of commands to be handled by this dispatch.
    pub fn new(store: Store, commands: Vec<&'static Command>) -> Self {
        Self {
            store,
            commands,
            editor: DefaultEditor::new().expect("Failed to create editor"),
        }
    }

    /// Returns a slice of all commands available in this dispatch.
    pub fn commands(&self) -> &[&'static Command] {
        &self.commands
    }

    /// Returns a reference to the underlying store.
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Returns a mutable reference to the underlying store.
    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    /// Runs a command by name with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `command_name` - The name of the command to run.
    /// * `args` - The arguments to pass to the command.
    ///
    /// # Errors
    ///
    /// Returns an error string if the command is not found or if the command handler fails.
    pub fn run(&mut self, command_name: &str, args: &[String]) -> Result<(), String> {
        match self.commands.iter().find(|c| c.name == command_name) {
            Some(command) => (command.run)(self, args),
            None => Err(format!("Command '{}' not found", command_name)),
        }
    }

    /// Parses a single line of input, extracts a command and arguments, and runs it.
    ///
    /// # Arguments
    ///
    /// * `line` - The raw line of user input.
    ///
    /// # Errors
    ///
    /// Returns an error string if no command is provided or if command execution fails.
    pub fn run_line(&mut self, line: &str) -> Result<(), String> {
        let tokens = shlex::Shlex::new(line).collect::<Vec<_>>();
        let command = tokens.get(0).ok_or("No command provided")?;
        let args = &tokens[1..];
        self.run(command, args)
    }

    /// Reads a line from standard input (using `rustyline`).
    ///
    /// # Errors
    ///
    /// Returns any `ReadlineError` that occurs during input reading.
    pub fn take_line(&mut self) -> Result<String, ReadlineError> {
        static PROMPT: &str = ">>> ";

        // Read a line from the user
        let line = self.editor.readline(PROMPT)?;

        // Add the line to the history
        self.editor.add_history_entry(&line)?;

        Ok(line)
    }

    /// Starts the main read-eval-print loop (REPL) for this dispatch.
    ///
    /// # Errors
    ///
    /// Returns an error string if reading from input fails unexpectedly.
    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let line = self.take_line().map_err(|e| e.to_string())?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Attempt to parse and run the command
            if let Err(e) = self.run_line(&line) {
                eprintln!("{}", e);
            }
        }
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::new(Store::default(), Command::all())
    }
}
