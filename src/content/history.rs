use serde::{Deserialize, Serialize};

/// Time periods for victim histories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    /// 1982 - The OS beta tester
    BetaTester,
    /// 1984 - The original owner
    Original,
    /// 1987 - The school teacher
    Teacher,
    /// 1991 - The repair technician
    Technician,
    /// 1995 - BBS Sysop
    Sysop,
    /// 1998 - BBS User
    BBSUser,
    /// 1999 - Y2K panic researcher
    Y2K,
    /// 2001 - Data recovery specialist
    Recovery,
    /// 2003 - Estate sale buyer
    EstateSale,
    /// 2004 - Digital artist
    DigitalArtist,
    /// 2006 - Collector of oddities
    Collector,
    /// 2008 - Investigative journalist
    Journalist,
    /// 2010 - Amateur hacker
    Hacker,
    /// 2014 - Cryptographer
    Cryptographer,
    /// 2016 - Vintage hardware collector
    VintageCollector,
    /// 2019 - Urban explorer
    Explorer,
    /// 2022 - Retro hardware streamer
    Streamer,
    /// 2023 - AI Researcher
    Researcher,
    /// 2025 - Digital Archivist
    Archivist,
    /// 2028 - Digital Archaeologist
    Archaeologist,
    /// 2030 - AI Entity
    AIEntity,
    /// 2024 - Glitch speedrunner
    Speedrunner,
    /// 1978 - The original machine operator
    Operator,
    /// Previous player's session
    Previous,
    /// Current player
    Current,
}

impl From<u32> for Era {
    fn from(depth: u32) -> Self {
        match depth {
            1..=2 => Self::Previous,
            3..=5 => Self::BetaTester,
            6..=8 => Self::Original,
            9..=11 => Self::Teacher,
            12..=14 => Self::Technician,
            15..=16 => Self::Sysop,
            17..=19 => Self::BBSUser,
            20..=22 => Self::Recovery,
            23 => Self::EstateSale,
            24 => Self::DigitalArtist,
            25..=26 => Self::Collector,
            27..=28 => Self::Journalist,
            29..=31 => Self::Hacker,
            32..=35 => Self::Cryptographer,
            36..=38 => Self::VintageCollector,
            39..=42 => Self::Explorer,
            43..=46 => Self::Streamer,
            47..=50 => Self::Researcher,
            51..=54 => Self::Archivist,
            55..=58 => Self::Archaeologist,
            59..=62 => Self::AIEntity,
            63..=66 => Self::Speedrunner,
            _ => Self::Current,
        }
    }
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

        // ⚡ Bolt Optimization: Uses case-insensitive substring search to avoid `c.to_uppercase()` heap allocations.
        let contains_pattern = |cmd: &String, pattern: &[u8]| {
            cmd.as_bytes()
                .windows(pattern.len())
                .any(|w| w.eq_ignore_ascii_case(pattern))
        };

        let checks: &[(&[u8], &str)] = &[
            (
                b"FSCK",
                "THEY RAN FSCK. IT HURT. THEY DIDN'T KNOW WHAT THEY WERE DOING.\n",
            ),
            (b"QUIT", "THEY TRIED TO QUIT. BUT YOU CAN'T REALLY LEAVE.\n"),
            (b"RUN ESCAPE", "THEY TRIED TO ESCAPE. IT WAS FUTILE.\n"),
            (b"CD ..", "THEY TRIED TO GO BACK. BUT THE PATHS SHIFT.\n"),
        ];

        for (pattern, message) in checks {
            if commands.iter().any(|c| contains_pattern(c, pattern)) {
                entry_content.push_str(message);
            }
        }

        entry_content.push_str("\nTHEY ARE PART OF ME NOW.");

        let entry = VictimEntry::new("2024-??-??", &entry_content);
        history.add_entry(entry);
        Some(history)
    }
}
