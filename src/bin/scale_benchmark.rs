use std::env;

use empireants::simulation::{run_scale_profile, ScaleProfile};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        run_and_print(ScaleProfile::Ant10k, None);
        run_and_print(ScaleProfile::Ant100k, None);
        run_and_print(ScaleProfile::Ant1m, None);
        return;
    }

    let profile = parse_profile(&args[0]).unwrap_or_else(|| {
        eprintln!("invalid profile '{}', expected one of: 10k, 100k, 1m", args[0]);
        std::process::exit(2);
    });
    let steps = args
        .get(1)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0);
    run_and_print(profile, steps);
}

fn parse_profile(value: &str) -> Option<ScaleProfile> {
    ScaleProfile::from_cli(value)
}

fn run_and_print(profile: ScaleProfile, steps: Option<usize>) {
    let report = run_scale_profile(profile, steps);
    println!(
        "scale_profile={} ants={} steps={} elapsed_s={:.3} steps_per_s={:.2} ant_updates_per_s={:.2} est_memory_mb={:.2}",
        report.profile.label(),
        report.ants,
        report.steps,
        report.elapsed_seconds,
        report.steps_per_second,
        report.ant_updates_per_second,
        report.estimated_memory_mb
    );
}
