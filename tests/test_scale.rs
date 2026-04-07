use empireants::simulation::ScaleProfile;

#[test]
fn parses_scale_profile_from_cli() {
    assert!(matches!(
        ScaleProfile::from_cli("10k"),
        Some(ScaleProfile::Ant10k)
    ));
    assert!(matches!(
        ScaleProfile::from_cli("100k"),
        Some(ScaleProfile::Ant100k)
    ));
    assert!(matches!(
        ScaleProfile::from_cli("1m"),
        Some(ScaleProfile::Ant1m)
    ));
    assert!(ScaleProfile::from_cli("unknown").is_none());
}

#[test]
fn profile_sizes_are_monotonic() {
    let p10 = ScaleProfile::Ant10k.ant_count();
    let p100 = ScaleProfile::Ant100k.ant_count();
    let p1m = ScaleProfile::Ant1m.ant_count();
    assert!(p10 < p100 && p100 < p1m);
}
