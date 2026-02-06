use fsck::content::{Era, VictimEntry, VictimHistory};

#[test]
fn test_create_victim_history() {
    let history = VictimHistory::new(Era::Original, "JAMIE", 1984);
    assert_eq!(history.name(), "JAMIE");
    assert_eq!(history.year(), 1984);
}

#[test]
fn test_history_has_entries() {
    let mut history = VictimHistory::new(Era::Original, "JAMIE", 1984);
    history.add_entry(VictimEntry::new(
        "1984-03-15",
        "Got this Apple IIe for my birthday! Setting everything up now.",
    ));
    assert_eq!(history.entries().len(), 1);
}

#[test]
fn test_history_by_era() {
    let h1 = VictimHistory::new(Era::Original, "JAMIE", 1984);
    let h2 = VictimHistory::new(Era::Technician, "MIKE", 1991);
    assert!(h1.era() == Era::Original);
    assert!(h2.era() == Era::Technician);
}
