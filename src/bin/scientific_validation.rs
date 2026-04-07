use std::path::PathBuf;

use empireants::simulation::{export_validation_csv, run_validation_suite};

fn main() {
    let artifact_dir = PathBuf::from("artifacts");
    if let Err(error) = std::fs::create_dir_all(&artifact_dir) {
        eprintln!("failed to create artifacts directory: {error}");
        std::process::exit(1);
    }

    let results = run_validation_suite();
    let output = artifact_dir.join("validation_report.csv");
    if let Err(error) = export_validation_csv(&output, &results) {
        eprintln!("failed to write validation report: {error}");
        std::process::exit(1);
    }

    println!("scientific validation complete");
    println!("rows={}", results.len());
    println!("artifact={}", output.display());
}
