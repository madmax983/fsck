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

#[test]
fn test_file_content_case_insensitive() {
    let lib = ContentLibrary::new();
    let content_upper = lib.file_content("README.TXT");
    let content_lower = lib.file_content("readme.txt");
    let content_mixed = lib.file_content("ReAdMe.TxT");
    assert_eq!(content_upper, content_lower);
    assert_eq!(content_upper, content_mixed);
}

#[test]
fn test_required_files_exist() {
    let lib = ContentLibrary::new();
    // Required files by spec
    assert!(lib.file_content("HELLO.BAS").is_some());
    assert!(lib.file_content("AUTOEXEC.BAS").is_some());
    assert!(lib.file_content("NOTES.TXT").is_some());
    assert!(lib.file_content("SYSTEM.LOG").is_some());
}

#[test]
fn test_hello_bas_content() {
    let lib = ContentLibrary::new();
    let content = lib.file_content("HELLO.BAS").unwrap();
    assert_eq!(content, "10 PRINT \"HELLO\"\n20 GOTO 10\n");
}

#[test]
fn test_generic_files_returns_tuples() {
    let lib = ContentLibrary::new();
    let files = lib.generic_files();
    assert!(!files.is_empty());
    // Each entry should be a tuple of (name, content)
    for (name, content) in files {
        assert!(!name.is_empty());
        assert!(!content.is_empty());
    }
}

#[test]
fn test_file_content_returns_none_for_nonexistent() {
    let lib = ContentLibrary::new();
    assert!(lib.file_content("NONEXISTENT.TXT").is_none());
}

#[test]
fn test_history_for_current_era_returns_none() {
    let lib = ContentLibrary::new();
    assert!(lib.history_for_era(Era::Current).is_none());
}

#[test]
fn test_new_history_has_empty_entries() {
    let history = VictimHistory::new(Era::Original, "TEST", 2024);
    assert_eq!(history.entries().len(), 0);
}

use fsck::content::DynamicContent;

#[test]
fn test_counter_increments() {
    let mut dynamic = DynamicContent::counter("HELLO\n");
    let first = dynamic.generate();
    let second = dynamic.generate();

    assert!(first.contains("HELLO"));
    assert!(second.contains("HELLO"));
    assert_ne!(first, second); // Should differ due to counter
}

#[test]
fn test_timestamp_changes() {
    let mut dynamic = DynamicContent::timestamp();
    let first = dynamic.generate();
    let second = dynamic.generate();

    // Both should be valid timestamps (rough check)
    assert!(first.contains("-"));
    assert!(second.contains("-"));
}

#[test]
fn test_corrupted_content() {
    let mut dynamic = DynamicContent::corrupted("HELLO", 0.3);
    let output = dynamic.generate();

    // Should have some corruption but still recognizable
    assert!(output.len() >= 5);
}
