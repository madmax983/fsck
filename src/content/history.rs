use serde::{Deserialize, Serialize};

/// Time periods for victim histories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    /// 1984 - The original owner
    Original,
    /// 1991 - The repair technician
    Technician,
    /// 1995 - BBS Sysop
    Sysop,
    /// 1998 - BBS User
    BBSUser,
    /// 1999 - Y2K panic researcher
    Y2K,
    /// 2003 - Estate sale buyer
    EstateSale,
    /// 2010 - Amateur hacker
    Hacker,
    /// 2014 - Cryptographer
    Cryptographer,
    /// 2019 - Urban explorer
    Explorer,
    /// 2022 - Retro hardware streamer
    Streamer,
    /// 2023 - AI Researcher
    Researcher,
    /// Previous player's session
    Previous,
    /// Current player
    Current,
}

/// A single diary/log entry from a victim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimEntry {
    date: String,
    content: String,
}

impl VictimEntry {
    /// Creates a new victim diary entry.
    ///
    /// # Arguments
    /// * `date` - The date of the entry (format: YYYY-MM-DD)
    /// * `content` - The diary entry text
    #[must_use]
    pub fn new(date: &str, content: &str) -> Self {
        Self {
            date: date.to_string(),
            content: content.to_string(),
        }
    }

    /// Returns the date of this entry.
    #[must_use]
    pub fn date(&self) -> &str {
        &self.date
    }

    /// Returns the content of this entry.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Complete history for one victim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimHistory {
    era: Era,
    name: String,
    year: u32,
    entries: Vec<VictimEntry>,
}

impl VictimHistory {
    /// Creates a new victim history.
    ///
    /// # Arguments
    /// * `era` - The time period this victim belongs to
    /// * `name` - The victim's name (will be normalized to uppercase)
    /// * `year` - The year this victim used the computer
    #[must_use]
    pub fn new(era: Era, name: &str, year: u32) -> Self {
        Self {
            era,
            name: name.to_uppercase(),
            year,
            entries: Vec::new(),
        }
    }

    /// Returns the era this victim belongs to.
    #[must_use]
    pub const fn era(&self) -> Era {
        self.era
    }

    /// Returns the victim's name (normalized to uppercase).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the year this victim used the computer.
    #[must_use]
    pub const fn year(&self) -> u32 {
        self.year
    }

    /// Returns all diary entries for this victim.
    #[must_use]
    pub fn entries(&self) -> &[VictimEntry] {
        &self.entries
    }

    /// Adds a new diary entry to this victim's history.
    ///
    /// # Arguments
    /// * `entry` - The diary entry to add
    pub fn add_entry(&mut self, entry: VictimEntry) {
        self.entries.push(entry);
    }

    /// Creates a victim history from a previous player's session commands.
    #[must_use]
    pub fn from_previous_session(commands: &[String]) -> Option<Self> {
        use std::fmt::Write;
        if commands.is_empty() {
            return None;
        }

        let mut history = Self::new(Era::Previous, "THE LAST ONE", 2024);
        // ⚡ Bolt Optimization: Pre-allocate String capacity to avoid multiple heap reallocations
        // when generating the previous player's history entry. 512 bytes safely covers the base text plus appending.
        let mut entry_content = String::with_capacity(512);
        entry_content.push_str("I WATCHED THEM PLAY. THEY TRIED TO UNDERSTAND.\n\n");

        let count = commands.len();
        write!(
            entry_content,
            "THEY ATTEMPTED {count} NOTABLE ACTIONS BEFORE THEY LEFT.\n\n"
        )
        .expect("Writing to String should not fail");

        if commands.iter().any(|c| c.to_uppercase().contains("FSCK")) {
            entry_content
                .push_str("THEY RAN FSCK. IT HURT. THEY DIDN'T KNOW WHAT THEY WERE DOING.\n");
        }
        if commands.iter().any(|c| c.to_uppercase().contains("QUIT")) {
            entry_content.push_str("THEY TRIED TO QUIT. BUT YOU CAN'T REALLY LEAVE.\n");
        }
        if commands
            .iter()
            .any(|c| c.to_uppercase().contains("RUN ESCAPE"))
        {
            entry_content.push_str("THEY TRIED TO ESCAPE. IT WAS FUTILE.\n");
        }
        if commands.iter().any(|c| c.to_uppercase().contains("CD ..")) {
            entry_content.push_str("THEY TRIED TO GO BACK. BUT THE PATHS SHIFT.\n");
        }

        entry_content.push_str("\nTHEY ARE PART OF ME NOW.");

        let entry = VictimEntry::new("2024-??-??", &entry_content);
        history.add_entry(entry);
        Some(history)
    }
}
