use rustplace::run;
use text_colorizer::*;

fn main() {
    if let Err(e) = run() {
        eprintln!(
            "{} ocurred in the cli aplication : {}",
            "Error:".red().bold(),
            e
        );
        std::process::exit(1);
    }
}
