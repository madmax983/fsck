use fsck::content::{ContentLibrary, Era, VictimEntry, VictimHistory};

#[test]
fn test_create_victim_history() {
    let history = VictimHistory::new(Era::Original, "JAMIE", 1984);
    assert_eq!(history.name(), "JAMIE");
    assert_eq!(history.year(), 1984);
}

#[test]
fn test_new_generic_files_exist() {
    let library = ContentLibrary::new();
    let files = library.generic_files();

    let has_story_bas = files.iter().any(|(n, _)| *n == "STORY.BAS");
    let has_machine_txt = files.iter().any(|(n, _)| *n == "MACHINE.TXT");
    let has_impossible_log = files.iter().any(|(n, _)| *n == "IMPOSSIBLE.LOG");
    let has_mind_txt = files.iter().any(|(n, _)| *n == "MIND.TXT");
    let has_echo_bas = files.iter().any(|(n, _)| *n == "ECHO.BAS");
    let has_sys_log_94 = files.iter().any(|(n, _)| *n == "SYS_LOG_94.TXT");
    let has_memory_bas = files.iter().any(|(n, _)| *n == "MEMORY.BAS");

    assert!(has_story_bas);
    assert!(has_machine_txt);
    assert!(has_impossible_log);
    assert!(has_mind_txt);
    assert!(has_echo_bas);
    assert!(has_sys_log_94);
    assert!(has_memory_bas);
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
    let h3 = VictimHistory::new(Era::VintageCollector, "GREG", 2016);
    assert!(h1.era() == Era::Original);
    assert!(h2.era() == Era::Technician);
    assert!(h3.era() == Era::VintageCollector);
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
    assert!(first.contains('-'));
    assert!(second.contains('-'));
}

#[test]
fn test_repeating_text_cycles() {
    let mut dynamic = DynamicContent::repeating(&["FIRST", "SECOND", "THIRD"]);

    let msg1 = dynamic.generate();
    assert_eq!(msg1, "FIRST");

    let msg2 = dynamic.generate();
    assert_eq!(msg2, "SECOND");

    let msg3 = dynamic.generate();
    assert_eq!(msg3, "THIRD");

    // Wraps around
    let msg4 = dynamic.generate();
    assert_eq!(msg4, "FIRST");
}

#[test]
fn test_repeating_text_empty() {
    let mut dynamic = DynamicContent::repeating(&[]);
    let msg = dynamic.generate();
    assert_eq!(msg, "");
}

#[test]
fn test_corrupted_content() {
    let mut dynamic = DynamicContent::corrupted("HELLO", 0.3);
    let output = dynamic.generate();

    // Should have some corruption but still recognizable
    assert!(output.len() >= 5);
}

#[test]
fn test_counter_handles_large_values() {
    let mut dynamic = DynamicContent::counter("X");
    for _ in 0..1000 {
        dynamic.generate();
    }
    let output = dynamic.generate();
    assert!(output.contains("1001"));
    assert!(output.len() < 100_000);
}

#[test]
fn test_corrupted_intensity_zero_no_corruption() {
    let mut dynamic = DynamicContent::corrupted("HELLO", 0.0);
    let output = dynamic.generate();
    assert_eq!(output.trim(), "HELLO");
}

#[test]
fn test_corrupted_deterministic() {
    let mut d1 = DynamicContent::corrupted("HELLO", 0.5);
    let mut d2 = DynamicContent::corrupted("HELLO", 0.5);
    assert_eq!(d1.generate(), d2.generate());
}

#[test]
fn test_corrupted_intensity_clamped() {
    let _d1 = DynamicContent::corrupted("HELLO", -0.5);
    let _d2 = DynamicContent::corrupted("HELLO", 1.5);
    // Should not panic, intensities clamped to 0.0-1.0
}

#[test]
fn test_patricia_history_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::EstateSale);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "PATRICIA");
    assert_eq!(history.year(), 2003);
    assert!(history.entries().len() >= 5);
}

#[test]
fn test_patricia_history_horror_escalation() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::EstateSale).unwrap();

    // Should have entries about estate sale, learning routine, can't turn off, relationship breakdown
    let all_content: String = history
        .entries()
        .iter()
        .map(fsck::content::VictimEntry::content)
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_content.contains("estate sale"));
    assert!(all_content.contains("John") || all_content.contains("relationship"));
    assert!(all_content.contains("can't turn it off") || all_content.contains("power button"));
}

#[test]
fn test_alex_2019_history_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Explorer);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "ALEX");
    assert_eq!(history.year(), 2019);
    assert!(history.entries().len() >= 4);
}

#[test]
fn test_alex_2019_reddit_format() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Explorer).unwrap();

    // Should have Reddit-style formatting
    let all_content: String = history
        .entries()
        .iter()
        .map(fsck::content::VictimEntry::content)
        .collect::<Vec<_>>()
        .join(" ");

    assert!(
        all_content.contains("r/urbanexploration")
            || all_content.contains("Reddit")
            || all_content.contains("Posted to")
    );
    assert!(all_content.contains("ALEX") || all_content.contains("IT KNOWS MY NAME"));
}

#[test]
fn test_sysop_history_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Sysop).unwrap();

    assert_eq!(history.era(), Era::Sysop);
    assert_eq!(history.name(), "KEVIN");
    assert_eq!(history.year(), 1995);
    assert!(!history.entries().is_empty());
}

#[test]
fn test_y2k_history_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Y2K).unwrap();

    assert_eq!(history.era(), Era::Y2K);
    assert_eq!(history.name(), "DAVID");
    assert_eq!(history.year(), 1999);
    assert!(!history.entries().is_empty());
}

#[test]
fn test_cryptographer_era_content() {
    let library = ContentLibrary::new();
    let cryptographer_history = library.history_for_era(Era::Cryptographer);

    assert!(cryptographer_history.is_some());
    let history = cryptographer_history.unwrap();

    assert_eq!(history.name(), "SARAH");
    assert_eq!(history.year(), 2014);
    assert_eq!(history.entries().len(), 4);

    let first_entry = &history.entries()[0];
    assert!(first_entry.date().starts_with("2014"));
}

#[test]
fn test_teacher_history_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Teacher);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "MRS. G");
    assert_eq!(history.year(), 1987);
    assert!(history.entries().len() >= 4);
}

#[test]
fn test_all_eras_have_unique_histories() {
    let lib = ContentLibrary::new();

    let original = lib.history_for_era(Era::Original);
    let teacher = lib.history_for_era(Era::Teacher);
    let technician = lib.history_for_era(Era::Technician);
    let sysop = lib.history_for_era(Era::Sysop);
    let bbs_user = lib.history_for_era(Era::BBSUser);
    let y2k = lib.history_for_era(Era::Y2K);
    let recovery = lib.history_for_era(Era::Recovery);
    let estate = lib.history_for_era(Era::EstateSale);
    let collector = lib.history_for_era(Era::Collector);
    let journalist = lib.history_for_era(Era::Journalist);
    let hacker = lib.history_for_era(Era::Hacker);
    let cryptographer = lib.history_for_era(Era::Cryptographer);
    let vintage_collector = lib.history_for_era(Era::VintageCollector);
    let explorer = lib.history_for_era(Era::Explorer);
    let streamer = lib.history_for_era(Era::Streamer);
    let researcher = lib.history_for_era(Era::Researcher);

    assert!(original.is_some());
    assert!(teacher.is_some());
    assert!(technician.is_some());
    assert!(sysop.is_some());
    assert!(bbs_user.is_some());
    assert!(y2k.is_some());
    assert!(recovery.is_some());
    assert!(estate.is_some());
    assert!(collector.is_some());
    assert!(journalist.is_some());
    assert!(hacker.is_some());
    assert!(cryptographer.is_some());
    assert!(vintage_collector.is_some());
    assert!(explorer.is_some());
    assert!(streamer.is_some());
    assert!(researcher.is_some());

    // Each should have different names
    assert_eq!(original.unwrap().name(), "JAMIE");
    assert_eq!(technician.unwrap().name(), "MIKE");
    assert_eq!(sysop.unwrap().name(), "KEVIN");
    assert_eq!(bbs_user.unwrap().name(), "MARCUS");
    assert_eq!(y2k.unwrap().name(), "DAVID");
    assert_eq!(estate.unwrap().name(), "PATRICIA");
    assert_eq!(collector.unwrap().name(), "ARTHUR");
    assert_eq!(journalist.unwrap().name(), "RACHEL");
    assert_eq!(hacker.unwrap().name(), "SAM");
    assert_eq!(cryptographer.unwrap().name(), "SARAH");
    assert_eq!(vintage_collector.unwrap().name(), "GREG");
    assert_eq!(explorer.unwrap().name(), "ALEX");
    assert_eq!(streamer.unwrap().name(), "CHRIS");
    assert_eq!(researcher.unwrap().name(), "ARIS");
}

#[test]
fn test_streamer_era_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Streamer);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "CHRIS");
    assert_eq!(history.year(), 2022);
    assert!(history.entries().len() >= 4);
}

#[test]
fn test_researcher_era_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Researcher);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "ARIS");
    assert_eq!(history.year(), 2023);
    assert!(history.entries().len() >= 4);
}

#[test]
fn test_recovery_history() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Recovery);
    assert!(history.is_some());

    let history = history.unwrap();
    assert_eq!(history.name(), "BEN");
    assert_eq!(history.year(), 2001);
    assert!(!history.entries().is_empty());
}

#[test]
fn test_archivist_era_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Archivist);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "ELIAS");
    assert_eq!(history.year(), 2025);
    assert!(history.entries().len() >= 3);
}

#[test]
fn test_collector_era_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Collector);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "ARTHUR");
    assert_eq!(history.year(), 2006);
    assert!(history.entries().len() >= 5);
}

#[test]
fn test_vintage_collector_era_exists() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::VintageCollector);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.name(), "GREG");
    assert_eq!(history.year(), 2016);
    assert!(history.entries().len() >= 2);
}
