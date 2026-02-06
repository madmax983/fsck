use serde::{Deserialize, Serialize};

/// Time periods for victim histories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    /// 1984 - The original owner
    Original,
    /// 1991 - The repair technician
    Technician,
    /// 2003 - Estate sale buyer
    EstateSale,
    /// 2019 - Urban explorer
    Explorer,
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
    pub fn new(date: &str, content: &str) -> Self {
        Self {
            date: date.to_string(),
            content: content.to_string(),
        }
    }

    pub fn date(&self) -> &str {
        &self.date
    }

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
    pub fn new(era: Era, name: &str, year: u32) -> Self {
        Self {
            era,
            name: name.to_uppercase(),
            year,
            entries: Vec::new(),
        }
    }

    pub fn era(&self) -> Era {
        self.era
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn entries(&self) -> &[VictimEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: VictimEntry) {
        self.entries.push(entry);
    }
}
