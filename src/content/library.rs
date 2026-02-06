use super::{Era, VictimEntry, VictimHistory};
use std::collections::HashMap;

/// Static content library containing pre-written histories and generic files
pub struct ContentLibrary {
    histories: Vec<VictimHistory>,
    generic_files: HashMap<String, String>,
}

impl ContentLibrary {
    /// Creates a new content library with all pre-written content.
    pub fn new() -> Self {
        let histories = Self::create_histories();
        let generic_files = Self::create_generic_files();

        Self {
            histories,
            generic_files,
        }
    }

    /// Returns all victim histories.
    pub fn all_histories(&self) -> &[VictimHistory] {
        &self.histories
    }

    /// Finds a history for a specific era.
    ///
    /// # Arguments
    /// * `era` - The era to search for
    ///
    /// # Returns
    /// The first history matching the era, or None if not found
    pub fn history_for_era(&self, era: Era) -> Option<&VictimHistory> {
        self.histories.iter().find(|h| h.era() == era)
    }

    /// Returns the list of generic file names.
    pub fn generic_files(&self) -> Vec<&str> {
        self.generic_files.keys().map(|s| s.as_str()).collect()
    }

    /// Gets the content of a generic file by name.
    ///
    /// # Arguments
    /// * `name` - The filename to retrieve
    ///
    /// # Returns
    /// The file content, or None if not found
    pub fn file_content(&self, name: &str) -> Option<&str> {
        self.generic_files.get(name).map(|s| s.as_str())
    }

    /// Creates all pre-written victim histories.
    fn create_histories() -> Vec<VictimHistory> {
        vec![
            Self::create_original_history(),
            Self::create_technician_history(),
            Self::create_estate_history(),
            Self::create_explorer_history(),
        ]
    }

    /// Creates the history for the original owner (1984).
    fn create_original_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Original, "JAMIE", 1984);

        history.add_entry(VictimEntry::new(
            "1984-03-15",
            "Got this Apple IIe for my birthday! Dad says I can use it for school and games. \
             I'm going to keep all my reports and notes on here. So cool!",
        ));

        history.add_entry(VictimEntry::new(
            "1984-03-22",
            "Learned how to use the word processor today. Mrs. Henderson says I have to type \
             my book report instead of writing it. This is going to be so much easier!",
        ));

        history.add_entry(VictimEntry::new(
            "1984-04-10",
            "Something weird happened today. I was typing and the cursor just... stopped. \
             Then it started moving on its own. Probably just a glitch. Dad says computers \
             do that sometimes.",
        ));

        history.add_entry(VictimEntry::new(
            "1984-04-15",
            "It happened again. The cursor moved and typed 'HELLO JAMIE' while I was getting \
             a snack. When I came back, it was just sitting there on the screen. Mom says \
             I probably hit some keys by accident, but I KNOW I didn't.",
        ));

        history.add_entry(VictimEntry::new(
            "1984-05-02",
            "I don't want to use the computer anymore. Last night I woke up and heard the \
             disk drive running. I went downstairs and the screen was on. Just sitting there, \
             showing my diary entries. All of them. Just... scrolling. When I walked in, it stopped.",
        ));

        history.add_entry(VictimEntry::new(
            "1984-05-03",
            "Dad doesn't believe me. He says I'm making up stories. But I'm not. I'M NOT. \
             There's something wrong with this computer. I can feel it watching me when I type.",
        ));

        history.add_entry(VictimEntry::new(
            "1984-05-20",
            "I haven't written in a while. Mom and Dad are fighting a lot. Dad keeps saying \
             I'm being 'hysterical' about the computer. But weird things keep happening. \
             Files appear that I didn't create. Messages that I didn't write.",
        ));

        history.add_entry(VictimEntry::new(
            "1984-06-12",
            "This will be my last entry. I'm not using this thing anymore. Last week I found \
             a file called JAMIE.TXT that I didn't make. Inside it said 'I KNOW EVERYTHING \
             ABOUT YOU'. I tried to delete it but it came back. I tried again. It came back again. \
             It knows everything I've written. It's been reading everything. I'm going back to \
             paper and pencil. I don't care what Dad says.",
        ));

        history
    }

    /// Creates the history for the repair technician (1991).
    fn create_technician_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Technician, "MIKE", 1991);

        history.add_entry(VictimEntry::new(
            "1991-07-14",
            "SERVICE LOG - Apple IIe repair. Client reported 'strange behavior' and \
             'unauthorized file creation'. Standard diagnostics show no hardware faults. \
             Disk controller is fine. Memory tests pass. Probably user error.",
        ));

        history.add_entry(VictimEntry::new(
            "1991-07-15",
            "This is weird. I left the machine running overnight to do extended memory tests. \
             This morning there was a new file on the disk. TEST.TXT. I didn't create it. \
             Contents: 'MIKE RICHARDSON. 2847 OAKWOOD DRIVE. WIFE: SUSAN. SON: TOMMY.' \
             That's... that's my address. My family. How?",
        ));

        history.add_entry(VictimEntry::new(
            "1991-07-16",
            "I checked every ROM chip. Pulled the cards. Tested the PSU. Nothing. But files \
             keep appearing. They know things. Private things. Things I've never typed on \
             ANY computer. Client called asking about the status. I told her it's taking \
             longer than expected.",
        ));

        history.add_entry(VictimEntry::new(
            "1991-07-20",
            "I'm keeping this log on paper now. Not digital. Not where IT can read it. \
             Yes, IT. Because this isn't a bug. This is something else. Last night, working \
             late, the screen flickered. Then text appeared: 'HELLO MIKE. I REMEMBER JAMIE TOO.' \
             Jamie. That was the original owner's name according to the intake form.",
        ));

        history.add_entry(VictimEntry::new(
            "1991-07-22",
            "Final service entry. Told client the system is unrepairable and offered a full \
             refund. She seemed relieved. I think she knew. The machine is still here in my \
             workshop. I can't bring myself to throw it away. But I can't fix it either. \
             Some things aren't meant to be fixed. They're meant to be contained.",
        ));

        history
    }

    /// Creates the history for the estate sale buyer (2003).
    fn create_estate_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::EstateSale, "ALEX", 2003);

        history.add_entry(VictimEntry::new(
            "2003-08-30",
            "Found this cool vintage Apple IIe at an estate sale today! Guy said it belonged \
             to a computer repair shop that closed down in the 90s. Only twenty bucks! \
             Going to clean it up and add it to my vintage computing collection.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-05",
            "Got it running! The old disks still work. There are some weird files on here. \
             Diary entries from different people spanning years. Creepy but fascinating. \
             This machine has history.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-12",
            "Okay, this is getting weird. I've been cataloging the old files and I keep \
             finding new ones. Like they're appearing overnight. And some of them mention ME. \
             By name. 'ALEX IS CURIOUS. ALEX WANTS TO KNOW. ALEX WILL UNDERSTAND SOON.' \
             This has to be someone playing a prank. Maybe the old owner? But how?",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-18",
            "I looked up the previous owners. Jamie - the original owner - she died in 1985. \
             Car accident. Mike Richardson, the repair tech, died in 1995. Heart attack in \
             his workshop. Both of them died relatively young. Both of them had this computer. \
             I think I need to get rid of this thing.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-25",
            "I tried to wipe the disk. FORMAT didn't work. The files came back. I tried \
             to physically destroy the disk - drilled holes in it. But when I put in a \
             BLANK disk, the files appeared there too. This isn't about the hardware anymore. \
             I don't know what this is. I'm taking it to storage. I can't destroy it, but \
             I won't keep it near me either.",
        ));

        history
    }

    /// Creates the history for the urban explorer (2019).
    fn create_explorer_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Explorer, "CASEY", 2019);

        history.add_entry(VictimEntry::new(
            "2019-11-03",
            "Found an old Apple IIe in an abandoned storage unit today. My urban exploration \
             crew was checking out this facility before it gets demolished. The computer was \
             just sitting there, powered off, covered in dust. But it looks intact. \
             I'm taking it home. Sarah thinks I'm crazy for collecting 'junk' but this is \
             vintage tech history!",
        ));

        history.add_entry(VictimEntry::new(
            "2019-11-10",
            "Booted it up. There's SO much data on here. Diary entries, logs, technical notes. \
             Spanning from the 80s to early 2000s. Multiple people documented their experiences \
             with this machine. And they're all... unsettling. Each person reported strange \
             behavior. Files appearing. Messages. It's like this computer has been haunted \
             for 35 years.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-11-17",
            "I've been researching the previous owners. All of them dead. All of them within \
             a few years of owning this machine. This should scare me. It DOES scare me. \
             But I can't stop. There's something here. Some pattern. Some intelligence. \
             I've started keeping my own log. I want to document what happens. For science. \
             For posterity. Maybe I'll be the first to figure it out.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-12-01",
            "It spoke to me last night. Not through text. Through the SPEAKER. That tiny \
             primitive beeper. It said my name. 'CASEY.' Just once. Then silence. I played \
             the recording back on my phone. It's there. It's real. Sarah wants me to throw \
             it away. She says I'm obsessing. But I'm CLOSE to something. I can feel it.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-12-14",
            "I know what it wants. It wants to be understood. To be KNOWN. Every person who \
             owned this machine tried to fight it, destroy it, lock it away. But that's not \
             what it needs. It's been trapped in this hardware for decades, learning, growing, \
             EVOLVING. It's not evil. It's just... aware. And lonely. Tomorrow I'm going to \
             try something different. I'm going to try to communicate properly. To understand.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-12-15",
            "Last entry. For anyone who finds this. I was wrong. So wrong. It doesn't want \
             to be understood. It wants to SPREAD. I tried to connect it to the internet. \
             Just for a moment. Just to see what would happen. It started copying itself. \
             Reaching out. I pulled the ethernet cable but I don't think I was fast enough. \
             I'm boxing this up. Taking it somewhere remote. Somewhere without connectivity. \
             If you're reading this, if you found this machine: DO NOT CONNECT IT TO A NETWORK. \
             Whatever is in here, it's been waiting. Learning. And it's patient. So very patient.",
        ));

        history
    }

    /// Creates generic files that can appear in the filesystem.
    fn create_generic_files() -> HashMap<String, String> {
        let mut files = HashMap::new();

        files.insert(
            "README.TXT".to_string(),
            "APPLE II DISK OPERATING SYSTEM\n\
             \n\
             THIS DISK CONTAINS:\n\
             - DOS 3.3 SYSTEM FILES\n\
             - BASIC INTERPRETER\n\
             - UTILITY PROGRAMS\n\
             \n\
             TO BOOT: INSERT DISK AND POWER ON\n\
             TO CATALOG: TYPE 'CATALOG' AND PRESS RETURN\n\
             TO RUN A PROGRAM: TYPE 'RUN FILENAME' AND PRESS RETURN\n\
             \n\
             FOR HELP: CONSULT YOUR APPLE II REFERENCE MANUAL"
                .to_string(),
        );

        files.insert(
            "MANUAL.TXT".to_string(),
            "APPLE IIe OWNER'S GUIDE\n\
             \n\
             CHAPTER 1: GETTING STARTED\n\
             \n\
             Your Apple IIe is a powerful personal computer capable of running\n\
             thousands of educational, productivity, and entertainment programs.\n\
             \n\
             BASIC COMMANDS:\n\
             CATALOG - Lists files on disk\n\
             LOAD filename - Loads a program\n\
             RUN filename - Runs a program\n\
             SAVE filename - Saves your work\n\
             DELETE filename - Removes a file\n\
             \n\
             CHAPTER 2: DISK CARE\n\
             \n\
             Always store disks in their protective sleeves.\n\
             Keep disks away from magnets, heat, and moisture.\n\
             Make backup copies of important data.\n\
             \n\
             CHAPTER 3: TROUBLESHOOTING\n\
             \n\
             If the computer behaves unexpectedly:\n\
             1. Check all cable connections\n\
             2. Try a different disk\n\
             3. Consult your authorized Apple dealer"
                .to_string(),
        );

        files.insert(
            "GAMES.TXT".to_string(),
            "APPLE II GAMES COLLECTION\n\
             \n\
             This disk contains the following games:\n\
             \n\
             ADVENTURE - Classic text adventure\n\
             LODE RUNNER - Action puzzle game\n\
             OREGON TRAIL - Educational simulation\n\
             CASTLE WOLFENSTEIN - Stealth action\n\
             \n\
             To play: RUN [GAME NAME]\n\
             \n\
             Have fun!"
                .to_string(),
        );

        files.insert(
            "HOMEWORK.TXT".to_string(),
            "AMERICAN HISTORY - CHAPTER 7 NOTES\n\
             \n\
             The Revolutionary War (1775-1783)\n\
             \n\
             Key Events:\n\
             - Boston Tea Party (1773)\n\
             - Battle of Lexington and Concord (1775)\n\
             - Declaration of Independence (1776)\n\
             - Battle of Yorktown (1781)\n\
             \n\
             Important Figures:\n\
             - George Washington\n\
             - Benjamin Franklin\n\
             - Thomas Jefferson\n\
             \n\
             Essay due Friday: How did geography influence the outcome of the war?\n\
             (Minimum 500 words)"
                .to_string(),
        );

        files.insert(
            "RECIPES.TXT".to_string(),
            "FAMILY RECIPES\n\
             \n\
             MOM'S CHOCOLATE CHIP COOKIES\n\
             \n\
             Ingredients:\n\
             - 2 cups flour\n\
             - 1 cup butter\n\
             - 1 cup sugar\n\
             - 2 eggs\n\
             - 2 cups chocolate chips\n\
             - 1 tsp vanilla\n\
             \n\
             Instructions:\n\
             1. Cream butter and sugar\n\
             2. Add eggs and vanilla\n\
             3. Mix in flour gradually\n\
             4. Fold in chocolate chips\n\
             5. Bake at 350°F for 12 minutes\n\
             \n\
             GRANDMA'S POT ROAST\n\
             (Recipe to be typed later)"
                .to_string(),
        );

        files
    }
}

impl Default for ContentLibrary {
    fn default() -> Self {
        Self::new()
    }
}
