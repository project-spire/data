use std::path::PathBuf;
use std::process::exit;

fn main() {
    let config = generator::Config {
        data_dir: PathBuf::from("../../src"),
        // gen_dir: out_dir.join("gen"),
        gen_dir: PathBuf::from("src/data"),
        verbose: true,
        dry_run: false,
    };

    if let Err(e) = config.generate() {
        eprintln!("Failed to generate: {}", e);
        exit(1);
    }
}