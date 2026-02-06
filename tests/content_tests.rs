use fsck::content::{ContentLibrary, Era, VictimEntry, VictimHistory};

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

#[test]
fn test_entry_getters() {
    let entry = VictimEntry::new("1984-03-15", "Test content");
    assert_eq!(entry.date(), "1984-03-15");
    assert_eq!(entry.content(), "Test content");
}

#[test]
fn test_name_uppercase_normalization() {
    let history = VictimHistory::new(Era::Original, "jamie", 1984);
    assert_eq!(history.name(), "JAMIE");
}

#[test]
fn test_multiple_entries() {
    let mut history = VictimHistory::new(Era::Original, "JAMIE", 1984);
    history.add_entry(VictimEntry::new("1984-03-15", "Entry 1"));
    history.add_entry(VictimEntry::new("1984-03-16", "Entry 2"));
    assert_eq!(history.entries().len(), 2);
    assert_eq!(history.entries()[0].date(), "1984-03-15");
    assert_eq!(history.entries()[1].date(), "1984-03-16");
}

#[test]
fn test_content_library_has_histories() {
    let lib = ContentLibrary::new();
    let histories = lib.all_histories();
    assert!(!histories.is_empty());
}

#[test]
fn test_get_history_by_era() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Original);
    assert!(history.is_some());
}

#[test]
fn test_library_has_generic_files() {
    let lib = ContentLibrary::new();
    let files = lib.generic_files();
    assert!(!files.is_empty());
}

#[test]
fn test_get_file_content_by_name() {
    let lib = ContentLibrary::new();
    let content = lib.file_content("README.TXT");
    assert!(content.is_some());
}
