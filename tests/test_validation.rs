use empireants::simulation::{run_validation_suite_with, AcoStrategy, ValidationScenario};

#[test]
fn validation_suite_generates_matrix_rows() {
    let rows = run_validation_suite_with(0.02, Some(32));
    assert_eq!(rows.len(), ValidationScenario::all().len() * AcoStrategy::all().len());
}

#[test]
fn validation_rows_have_nonzero_steps() {
    let rows = run_validation_suite_with(0.01, Some(16));
    assert!(rows.iter().all(|row| row.steps >= 1));
}
