use anyhow::Result;

pub mod cmd;
pub mod store;

fn main() -> Result<()> {
    if let Err(e) = cmd::Dispatch::default().start() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
