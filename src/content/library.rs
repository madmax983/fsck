#![allow(clippy::too_many_lines)]
use super::{Era, VictimEntry, VictimHistory};

/// Static content library containing pre-written histories and generic files
pub struct ContentLibrary {
    histories: Vec<VictimHistory>,
    generic_files: Vec<(&'static str, &'static str)>,
}

impl ContentLibrary {
    /// Creates a new content library with all pre-written content.
    #[must_use]
    pub fn new() -> Self {
        let histories = Self::create_histories();
        let generic_files = Self::create_generic_files();

        Self {
            histories,
            generic_files,
        }
    }

    /// Returns all victim histories.
    #[must_use]
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
    #[must_use]
    pub fn history_for_era(&self, era: Era) -> Option<&VictimHistory> {
        self.histories.iter().find(|h| h.era() == era)
    }

    /// Returns the list of generic files as (name, content) tuples.
    #[must_use]
    pub fn generic_files(&self) -> &[(&'static str, &'static str)] {
        &self.generic_files
    }

    /// Gets the content of a generic file by name (case-insensitive).
    ///
    /// # Arguments
    /// * `name` - The filename to retrieve
    ///
    /// # Returns
    /// The file content, or None if not found
    #[must_use]
    pub fn file_content(&self, name: &str) -> Option<&'static str> {
        self.generic_files
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, content)| *content)
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

    /// Creates the history for the estate sale buyer (2003) - Patricia.
    fn create_estate_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::EstateSale, "PATRICIA", 2003);

        history.add_entry(VictimEntry::new(
            "2003-09-14",
            "Found this at an estate sale for $20. Seller seemed eager to get \
             rid of it. Said something about it being cursed but I think \
             they were joking. Boots up fine.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-20",
            "There are files on here from before I bought it. Personal stuff. \
             Diary entries? Some of them mention my address. But I just moved here. \
             How would anyone know?",
        ));

        history.add_entry(VictimEntry::new(
            "2003-09-27",
            "It's learning my routine. Files appear matching my schedule. \
             WORK.TXT appears at 8am. LUNCH.TXT at noon. \
             Today it created ARGUMENT.TXT right after I fought with John. \
             We were in the other room. The computer was off.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-10-05",
            "I can't turn it off. The power button doesn't work. Unplugging it \
             doesn't work - it stays on. John says I'm crazy but he won't \
             come into the room anymore.",
        ));

        history.add_entry(VictimEntry::new(
            "2003-10-12",
            "Taking it to the dump today. John is leaving me. He says he can't \
             live with someone who's losing their mind. Maybe he's right. \
             Maybe it's me. Maybe I'm imagining all of this.",
        ));

        history
    }

    /// Creates the history for the urban explorer (2019) - Alex.
    fn create_explorer_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Explorer, "ALEX", 2019);

        history.add_entry(VictimEntry::new(
            "2019-06-03",
            "[Posted to r/urbanexploration]\n\
             Found an Apple IIe in an abandoned house today. Still works!\n\
             Boot screen is normal but the filesystem is MASSIVE. Like,\n\
             hundreds of nested directories. Going to explore more tomorrow.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-06-04",
            "[Edited: 2019-06-05 3:47 AM]\n\
             ~~Something is wrong with this computer~~\n\
             Never mind, just corrupted sectors. Nothing to see here.",
        ));

        history.add_entry(VictimEntry::new(
            "2019-06-07",
            "[Deleted post - recovered from cache]\n\
             IT KNOWS MY NAME. It's never seen my name. I never typed my name.\n\
             There's a directory called ALEX and inside are photos of my apartment.\n\
             I NEVER CONNECTED A CAMERA. I NEVER UPLOADED ANYTHING.\n\
             This is my throwaway account. How does it know who I am?",
        ));

        history.add_entry(VictimEntry::new(
            "2019-06-10",
            "[Account deleted]\n\
             [Final edit before deletion]\n\
             If you find this computer, don't boot it up. Just destroy it.\n\
             Smash the disk drive. Burn the chips. I'm serious.\n\
             It's not a computer anymore. It's something else.\n\
             And it's very, very patient.",
        ));

        history
    }

    /// Creates generic files that can appear in the filesystem.
    fn create_generic_files() -> Vec<(&'static str, &'static str)> {
        vec![
            // Required files by spec
            ("HELLO.BAS", "10 PRINT \"HELLO\"\n20 GOTO 10\n"),
            (
                "AUTOEXEC.BAS",
                "10 REM AUTO START\n20 PRINT \"LOADING...\"\n",
            ),
            (
                "NOTES.TXT",
                "Remember to run FSCK regularly\nSome sectors are showing errors\nWill investigate deeper directories tomorrow\n",
            ),
            (
                "SYSTEM.LOG",
                "1984-03-15 12:34:56 BOOT\n1984-03-15 12:35:01 USER LOGIN\n1984-03-15 12:35:45 DISK ERROR SECTOR 23\n1984-03-15 12:35:45 REPAIR FAILED\n",
            ),
            // Additional generic files
            (
                "README.TXT",
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
                 FOR HELP: CONSULT YOUR APPLE II REFERENCE MANUAL",
            ),
            (
                "MANUAL.TXT",
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
                 3. Consult your authorized Apple dealer",
            ),
            (
                "GAMES.TXT",
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
                 Have fun!",
            ),
            (
                "HOMEWORK.TXT",
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
                 (Minimum 500 words)",
            ),
            (
                "RECIPES.TXT",
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
                 (Recipe to be typed later)",
            ),
        ]
    }
}

impl Default for ContentLibrary {
    fn default() -> Self {
        Self::new()
    }
}
