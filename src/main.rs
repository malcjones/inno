pub mod cmd;
pub mod store;

fn main() {
    if let Err(e) = cmd::Dispatch::default().start() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
