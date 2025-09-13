use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = generator::Config {
        data_dir: PathBuf::from("../../src"),
        // gen_dir: out_dir.join("gen"),
        gen_dir: PathBuf::from("src"),
        dry_run: false,
    };
    config.generate()?;

    Ok(())
}