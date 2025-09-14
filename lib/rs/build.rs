use std::path::PathBuf;
use std::process::exit;

fn main() {
    let config = generator::Config {
        schema_dir: PathBuf::from("../../schema"),
        src_dir: PathBuf::from("../../src"),
        gen_dir: PathBuf::from("src/data"),
        verbose: true,
        dry_run: false,
    };

    println!("cargo:rerun-if-changed={}", config.schema_dir.display());

    if let Err(e) = config.generate() {
        eprintln!("Failed to generate: {}", e);
        exit(1);
    }
}