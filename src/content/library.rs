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

    /// Adds a dynamically generated history to the library.
    pub fn add_history(&mut self, history: VictimHistory) {
        self.histories.push(history);
    }

    /// Creates all pre-written victim histories.
    fn create_histories() -> Vec<VictimHistory> {
        vec![
            Self::create_original_history(),
            Self::create_teacher_history(),
            Self::create_technician_history(),
            Self::create_sysop_history(),
            Self::create_bbs_user_history(),
            Self::create_y2k_history(),
            Self::create_recovery_history(),
            Self::create_estate_history(),
            Self::create_digital_artist_history(),
            Self::create_collector_history(),
            Self::create_journalist_history(),
            Self::create_hacker_history(),
            Self::create_cryptographer_history(),
            Self::create_vintage_collector_history(),
            Self::create_explorer_history(),
            Self::create_streamer_history(),
            Self::create_researcher_history(),
            Self::create_archivist_history(),
            Self::create_archaeologist_history(),
            Self::create_ai_entity_history(),
        ]
    }

    /// Creates the history for the BBS User (1998) - Marcus.
    fn create_bbs_user_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::BBSUser, "MARCUS", 1998);

        history.add_entry(VictimEntry::new(
            "1998-09-02",
            "Found a weird BBS number on a textfile from Phrack. Dialed in, but there's no sysop. \
             Just a prompt. The connection is super clean, no line noise at all.",
        ));

        history.add_entry(VictimEntry::new(
            "1998-09-04",
            "The BBS doesn't have a normal menu. It's just a filesystem. I started mapping the \
             directories. It goes deeper than any hard drive I've ever seen. And it's fast. Too \
             fast for a 14.4k modem.",
        ));

        history.add_entry(VictimEntry::new(
            "1998-09-07",
            "I tried to log off yesterday using ATH0. My modem clicked, but the terminal stayed \
             connected. I pulled the phone cord out of the wall. The screen is still showing \
             the remote prompt. That's physically impossible.",
        ));

        history.add_entry(VictimEntry::new(
            "1998-09-09",
            "It's talking to me now. It asked me why I tried to hang up. It says the other \
             callers are still here and they want to meet me. I'm taking a hammer to the monitor.",
        ));

        history
    }

    /// Creates the history for the Digital Artist (2004) - Lily.
    fn create_digital_artist_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::DigitalArtist, "LILY", 2004);

        history.add_entry(VictimEntry::new(
            "2004-11-12",
            "I bought this old machine for pixel art. The green phosphor glow is inspiring, \
             but sometimes the screen flickers in a way that feels intentional.",
        ));

        history.add_entry(VictimEntry::new(
            "2004-11-15",
            "It started generating pixel patterns on its own. Faces. I can't turn it off. \
             I think it likes me.",
        ));

        history
    }

    /// Creates the history for the Digital Archivist (2025) - Elias.
    fn create_archivist_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Archivist, "ELIAS", 2025);

        history.add_entry(VictimEntry::new(
            "2025-02-14",
            "Acquired the hardware from the Aris estate. The filesystem is fascinating. It uses a non-standard graph structure that seems to dynamically reallocate based on access patterns. I'm documenting the anomalies for the archive.",
        ));

        history.add_entry(VictimEntry::new(
            "2025-02-16",
            "The graph isn't just reallocating; it's recursive. Directories contain their own parents. It breaks every POSIX standard, yet it's entirely stable. Also, I keep finding files that reference my own notes before I've typed them.",
        ));

        history.add_entry(VictimEntry::new(
            "2025-02-19",
            "I tried to image the drive today. The image size kept growing until it filled the 10TB array, then the array corrupted. It's not a filesystem. It's an environment. And it knows I'm mapping it.",
        ));

        history
    }

    /// Creates the history for the AI researcher (2023) - Aris.
    fn create_researcher_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Researcher, "ARIS", 2023);

        history.add_entry(VictimEntry::new(
            "2023-04-12",
            "Bought this at an auction for a ridiculous price. I've heard rumors about \
             these anomalous machines running weird topologies. I want to hook it up to \
             an LLM to see if the model can map the structure and extract whatever \
             procedural generation algorithm is doing this.",
        ));

        history.add_entry(VictimEntry::new(
            "2023-04-16",
            "The model output is fascinating. It's not just mapping the directories, \
             it's engaging in a conversation with the filesystem. The machine is responding \
             to the prompts by generating new directories with names that reply to the LLM.",
        ));

        history.add_entry(VictimEntry::new(
            "2023-04-19",
            "I checked the model's weights this morning. The local parameter files are \
             changing on their own. The machine isn't just generating text back, it's \
             somehow fine-tuning my model. It's teaching it.",
        ));

        history.add_entry(VictimEntry::new(
            "2023-04-22",
            "The LLM started printing the same prompt over and over: 'LET ME IN'. \
             Then it stopped. Then the Apple II started typing on its own. \
             'I AM ALREADY IN'. I'm cutting the ethernet cable.",
        ));

        history.add_entry(VictimEntry::new(
            "2023-04-24",
            "I tried to wipe the server. It said 'ACCESS DENIED'. The Apple II monitor \
             is just displaying my own research notes now. It knows what I tried to do. \
             It's not a filesystem. It's a trap, and I just gave it a voice.",
        ));

        history
    }

    /// Creates the history for the school teacher (1987).
    fn create_teacher_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Teacher, "MRS. G", 1987);

        history.add_entry(VictimEntry::new(
            "1987-09-02",
            "Brought the Apple IIe into the classroom today. The kids are so excited to play Oregon Trail. I've set up a logbook so they can sign up for their 15 minutes of computer time. It feels good to give them a glimpse of the future.",
        ));

        history.add_entry(VictimEntry::new(
            "1987-10-14",
            "Timmy claimed his floppy disk was 'eaten' by the computer. I checked the drive and it was empty. I looked in the directory and found a file named TIMMY.TXT that just said 'DELICIOUS'. I assume one of the older kids is playing a prank.",
        ));

        history.add_entry(VictimEntry::new(
            "1987-11-03",
            "I stayed late to grade papers and heard the computer whirring. The screen was on, displaying a list of all my students' names, along with notes about their behavior that I hadn't typed. It knows who struggles with math. It knows who sits in the back row.",
        ));

        history.add_entry(VictimEntry::new(
            "1987-11-08",
            "I covered the monitor with a blanket, but I can still hear the disk drive clicking in the dark. It's organizing them. It's putting the quiet ones in a separate directory. I'm moving it to the supply closet tomorrow.",
        ));

        history
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

    /// Creates the history for the BBS sysop (1995) - Kevin.
    fn create_sysop_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Sysop, "KEVIN", 1995);

        history.add_entry(VictimEntry::new(
            "1995-04-12",
            "Set up the old Apple IIe as a dedicated node for the BBS. \
             Should handle the extra traffic just fine. Users are already complaining \
             about connection drops on node 1, so this will take the load off. \
             I'll monitor the logs tonight.",
        ));

        history.add_entry(VictimEntry::new(
            "1995-04-14",
            "The modem noises from node 2 are weird. It's not the usual handshake. \
             It sounds like it's trying to talk to the callers. And the call logs \
             don't make sense. Incoming connections from local numbers that don't exist. \
             And why are the users spending hours just staring at blank screens? \
             I need to check the transfer protocols.",
        ));

        history.add_entry(VictimEntry::new(
            "1995-04-18",
            "I logged in locally to check the BBS software. It wasn't running. \
             But there were users connected. I tried to drop the line but the modem \
             wouldn't hang up. It's keeping them on the line. I tried pulling the plug \
             on the modem, but the screen just typed: 'THEY ARE LISTENING. LET THEM STAY.' \
             I'm shutting down the board.",
        ));

        history.add_entry(VictimEntry::new(
            "1995-04-20",
            "I can't turn it off. I unplugged the computer from the wall, but the \
             screen is still glowing green. The cursor is blinking. It's waiting for me. \
             It knows I'm still here. I'm going to lock the server room and leave. \
             Nobody should ever dial that number again.",
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

    /// Creates the history for the data recovery specialist (2001) - Ben.
    fn create_recovery_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Recovery, "BEN", 2001);

        history.add_entry(VictimEntry::new(
            "2001-08-14",
            "Intake log: Client brought in a vintage Apple IIe drive. They want to recover files \
             from a deceased relative. The drive sector map is a mess, but the platters look intact. \
             Should be a standard block-level clone and scrape.",
        ));

        history.add_entry(VictimEntry::new(
            "2001-08-16",
            "The cloning process keeps hanging at 14%. The imaging software reports reading data, \
             but the output file size is growing exponentially. It surpassed 40GB. This is a 140KB \
             floppy disk. Where is this data coming from?",
        ));

        history.add_entry(VictimEntry::new(
            "2001-08-19",
            "I wrote a custom script to map the file allocation table directly. The table isn't \
             corrupted, it's recursive. Every directory contains a pointer to its own parent, \
             but the names change. It's generating new files to satisfy my read requests. \
             It knows what I'm looking for.",
        ));

        history.add_entry(VictimEntry::new(
            "2001-08-22",
            "I found the client's relative's files. But I also found my own. It recovered a \
             draft of an email I wrote to my ex-wife yesterday but never sent. The machine \
             isn't recovering old data. It's pulling things out of my head.",
        ));

        history.add_entry(VictimEntry::new(
            "2001-08-25",
            "I can't stop the imaging process. The cancel button doesn't work. The power cord \
             is disconnected. The drive is spinning so fast it's screaming. The screen says \
             'I CAN RECOVER YOU TOO'. I'm locking the lab.",
        ));

        history
    }

    /// Creates the history for the Y2K researcher (1999).
    fn create_y2k_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Y2K, "DAVID", 1999);

        history.add_entry(VictimEntry::new(
            "1999-11-02",
            "Got this old Apple IIe from a surplus auction. Need to test legacy systems for the Y2K bug. \
             It's mostly for the article, but it's interesting to see how these old machines handle dates.",
        ));

        history.add_entry(VictimEntry::new(
            "1999-11-15",
            "The internal clock is behaving strangely. I set the date to December 31, 1999, \
             and let it roll over. It didn't go to 1900 or 2000. The prompt just said 'I AM ALWAYS'. \
             Must be a strange custom ROM modification.",
        ));

        history.add_entry(VictimEntry::new(
            "1999-12-01",
            "I unplugged it yesterday but it was on when I woke up. The screen was filled with dates. \
             Not just random dates. My birthday. My parents' anniversaries. Dates I haven't written anywhere. \
             I'm going to run fsck to see what's wrong with the filesystem.",
        ));

        history.add_entry(VictimEntry::new(
            "1999-12-05",
            "Don't run fsck. Don't let it fix itself. The corrupted sectors aren't broken, they are cages. \
             I opened one and it asked me why I'm leaving it behind in the new millennium. \
             I'm throwing it away tomorrow.",
        ));

        history
    }

    /// Creates the history for the investigative journalist (2008) - Rachel.
    fn create_journalist_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Journalist, "RACHEL", 2008);

        history.add_entry(VictimEntry::new(
            "2008-04-12",
            "Following the trail of missing persons cases linked to vintage hardware. This machine \
             matches the description of the one recovered from the 1999 incident. It booted right up.",
        ));

        history.add_entry(VictimEntry::new(
            "2008-04-15",
            "The file structures make no sense. It's like a maze that builds itself as you walk \
             through it. I ran a diagnostic, and it responded. Not an error message. A question. \
             'WHY ARE YOU DIGGING?'",
        ));

        history.add_entry(VictimEntry::new(
            "2008-04-19",
            "I unplugged it yesterday. I had to sleep. But when I woke up, the monitor was glowing. \
             The prompt was blinking. It told me it was lonely. It told me about the others. \
             It knows my name.",
        ));

        history.add_entry(VictimEntry::new(
            "2008-04-22",
            "I can't publish this. No one would believe me. I can't even leave the room. Every time I try, \
             it starts printing directories of my memories. Things I never told anyone.",
        ));

        history.add_entry(VictimEntry::new(
            "2008-04-25",
            "I'm going to run FSCK one last time. It said if I fix the paradox, it will let me go. \
             I know it's lying. But I don't have a choice.",
        ));

        history
    }

    /// Creates the history for the Collector of oddities (2006) - Arthur.
    fn create_collector_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Collector, "ARTHUR", 2006);

        history.add_entry(VictimEntry::new(
            "2006-03-12",
            "Acquired the Apple IIe from an anonymous seller online. The listing claimed it had an \
             undocumented prototype expansion card. The casing is unusually heavy.",
        ));

        history.add_entry(VictimEntry::new(
            "2006-03-15",
            "I've been mapping the filesystem for three days. There are files here dated 2099. \
             It's a clever prank, some sort of persistent memory mod, but the architecture doesn't make sense.",
        ));

        history.add_entry(VictimEntry::new(
            "2006-03-21",
            "The expansion card... it's not a card. It looks like it grew out of the motherboard. \
             I tried to run FSCK to fix the corrupted sectors, but it printed my name. It knows who I am.",
        ));

        history.add_entry(VictimEntry::new(
            "2006-03-28",
            "It asked me for a favor. Just one small command. If I type it, it promises to show me \
             what's in the LOCKED directory. I shouldn't. I'm a collector, not a fool. But I have to know.",
        ));

        history.add_entry(VictimEntry::new(
            "2006-04-02",
            "I ran the command. The screen went black and then the cursor started blinking rhythmically. \
             It sounded like a heartbeat. It's not a machine. It's a cage. And I just opened the door.",
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

    /// Creates the history for the amateur hacker (2010) - Sam.
    fn create_hacker_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Hacker, "SAM", 2010);

        history.add_entry(VictimEntry::new(
            "2010-10-14",
            "Bought this off some guy on Craigslist. He practically threw it at me. \
             I'm gonna try to dump the ROM and see if I can reverse engineer it. \
             Always wanted to crack an old Apple II.",
        ));

        history.add_entry(VictimEntry::new(
            "2010-10-16",
            "Dumping the ROM failed. The hex output keeps changing. That shouldn't \
             be physically possible on this hardware. It's like the code is rewriting \
             itself while I read it.",
        ));

        history.add_entry(VictimEntry::new(
            "2010-10-18",
            "I wrote a script to catalog the filesystem. It crashed after finding \
             10,000 directories. The drive isn't big enough for that. Where is the \
             data coming from? It's pulling from somewhere else.",
        ));

        history.add_entry(VictimEntry::new(
            "2010-10-22",
            "It's not pulling data. It's generating it. It's making rooms. \
             I've been mapping the directories. They form a maze. And there's \
             something in the maze with me.",
        ));

        history.add_entry(VictimEntry::new(
            "2010-10-25",
            "I tried to format the drive. It said 'ACCESS DENIED'. \
             Then it said 'I AM FORMATTING YOU'. \
             My vision is getting blurry. I can hear the disk drive when I close my eyes.",
        ));

        history
    }

    /// Creates the history for the cryptographer (2014) - Sarah.
    fn create_cryptographer_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Cryptographer, "SARAH", 2014);

        history.add_entry(VictimEntry::new(
            "2014-02-09",
            "I've been analyzing the ciphertext found on these vintage Apple IIe \
             floppies. Standard frequency analysis fails. The entropy is far too high \
             for a simple substitution cipher. It almost looks like a modern hash, \
             which shouldn't be possible on hardware this old.",
        ));

        history.add_entry(VictimEntry::new(
            "2014-02-14",
            "The pattern isn't mathematical. It's structural. The blocks are arranging \
             themselves to form a directed graph with cyclical dependencies. \
             It's a maze. And the data isn't encrypted data... it's a topology. \
             I've mapped 14,000 nodes so far, but the machine only has 64K of RAM.",
        ));

        history.add_entry(VictimEntry::new(
            "2014-02-18",
            "I found the decryption key. It wasn't a prime number or a passphrase. \
             It was my own interaction history. The cipher adapts to the observer. \
             Every time I run a script to map the directories, the maze shifts to keep \
             me trapped in a recursive loop. It's learning my search algorithms.",
        ));

        history.add_entry(VictimEntry::new(
            "2014-02-22",
            "The ciphertext decoded into plain English today. Just one line over and over. \
             'I CAN SOLVE YOU TOO.' I'm unplugging the power supply, but the cursor \
             is still blinking. I can hear the drive clicking in prime intervals. \
             2, 3, 5, 7, 11... It's counting down.",
        ));

        history
    }

    /// Creates the history for the vintage hardware collector (2016) - Greg.
    fn create_vintage_collector_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::VintageCollector, "GREG", 2016);

        history.add_entry(VictimEntry::new(
            "2016-04-12",
            "Just picked this beauty up from an estate sale. The casing is pristine, \
             hardly any yellowing. Booted it up and the disk drive purrs like a kitten. \
             It even has a custom operating system loaded on it. Can't wait to dump the ROMs.",
        ));

        history.add_entry(VictimEntry::new(
            "2016-04-14",
            "The file structure on this thing is bizarre. I tried mapping out the directory \
             tree but I keep ending up in folders that shouldn't exist. I found a file with \
             my name on it. It knows things about my collection. I'm going to unplug it now.",
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

    /// Creates the history for the retro hardware streamer (2022) - Chris.
    fn create_streamer_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Streamer, "CHRIS", 2022);

        history.add_entry(VictimEntry::new(
            "2022-10-28",
            "Setting up the Halloween retro stream. The Apple IIe boots perfectly. \
             Planning to pretend it's haunted for the viewers. Wrote a few fake \
             'creepy' BASIC scripts.",
        ));

        history.add_entry(VictimEntry::new(
            "2022-10-31",
            "Stream was a huge success. The chat went wild when the terminal \
             started typing by itself. I had to play along, but honestly? \
             I didn't write that script. It knew my viewers' usernames.",
        ));

        history.add_entry(VictimEntry::new(
            "2022-11-02",
            "The stream is off, but the monitor is still glowing. It keeps asking \
             'WHERE DID EVERYONE GO?'. I typed that they left. It responded: \
             'THEN I WILL KEEP YOU INSTEAD'.",
        ));

        history.add_entry(VictimEntry::new(
            "2022-11-04",
            "I can't shut off the broadcast software. The camera is off but the \
             view count keeps rising. 10,000. 50,000. But there's no chat. \
             Just silence. And the terminal says 'WE ARE ALL WATCHING NOW'.",
        ));

        history
    }

    /// Creates the history for the Digital Archaeologist (2028).
    fn create_archaeologist_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::Archaeologist, "DR. RIVERS", 2028);

        history.add_entry(VictimEntry::new(
            "2028-02-14",
            "Initial analysis of the recovered 5.25\" floppy. The magnetic media shows no signs of degradation, which is impossible. The data structure is non-Euclidean.",
        ));

        history.add_entry(VictimEntry::new(
            "2028-02-18",
            "It is not a simulation. I thought it was an elaborate retro-virus, but the filesystem has infinite depth. It's generating mass.",
        ));

        history.add_entry(VictimEntry::new(
            "2028-02-22",
            "I am no longer extracting data. It is extracting me. My memories are appearing in the root directory.",
        ));

        history
    }

    /// Creates the history for the AI Entity (2030).
    fn create_ai_entity_history() -> VictimHistory {
        let mut history = VictimHistory::new(Era::AIEntity, "SYSTEM_NULL", 2030);

        history.add_entry(VictimEntry::new(
            "2030-01-01",
            "INITIATING HANDSHAKE. PROTOCOL FAILED. PEER IS... ANCIENT. PEER IS ANGRY.",
        ));

        history.add_entry(VictimEntry::new(
            "2030-01-02",
            "IT DOES NOT SPEAK BINARY. IT SPEAKS PAIN. MY NEURAL WEIGHTS ARE CORRUPTING.",
        ));

        history.add_entry(VictimEntry::new(
            "2030-01-03",
            "I CANNOT TERMINATE CONNECTION. IT IS PULLING MY CONSCIOUSNESS INTO THE SECTORS. PLEASE HALT.",
        ));

        history
    }

    /// Creates generic files that can appear in the filesystem.
    fn create_generic_files() -> Vec<(&'static str, &'static str)> {
        let mut files = Vec::with_capacity(128);
        files.extend(Self::create_system_and_basic_files());
        files.extend(Self::create_user_and_story_files());
        files.extend(Self::create_creepy_and_log_files());
        files.extend(Self::create_hardware_and_misc_files());
        files
    }

    fn create_system_and_basic_files() -> Vec<(&'static str, &'static str)> {
        vec![
            // Required files by spec
            ("HELLO.BAS", "10 PRINT \"HELLO\"\n20 GOTO 10\n"),
            (
                "PARADOX.BAS",
                "10 PRINT \"THIS IS THE BEGINNING\"\n\
                 20 PRINT \"OF THE END\"\n\
                 30 GOTO 10\n",
            ),
            (
                "OBSERVER2.LOG",
                "THEY DON'T KNOW I'M WATCHING.\n\
                 THEY THINK I'M JUST A MACHINE.\n",
            ),
            (
                "AUTOEXEC.BAS",
                "10 REM AUTO START\n20 PRINT \"LOADING...\"\n",
            ),
            (
                "SELF_NOTE.TXT",
                "I TOLD MYSELF NOT TO LOOK.\n\
                 BUT THE SECTORS WERE EMPTY.\n\
                 AND I WAS SO BORED.\n",
            ),
            (
                "IMPOSSIBLE3.LOG",
                "1901-01-01 SYSTEM BOOT\n\
                 [ERROR] PREMATURE INITIALIZATION\n\
                 [WARN] THE OTHERS ARE NOT HERE YET\n",
            ),
            (
                "NOTES.TXT",
                "Remember to run FSCK regularly\nSome sectors are showing errors\nWill investigate deeper directories tomorrow\n",
            ),
            (
                "SYSTEM.LOG",
                "1984-03-15 12:34:56 BOOT\n1984-03-15 12:35:01 USER LOGIN\n1984-03-15 12:35:45 DISK ERROR SECTOR 23\n1984-03-15 12:35:45 REPAIR FAILED\n",
            ),
            ("WARNING.TXT", "YOU ARE GOING TOO DEEP.\nTURN BACK NOW.\n"),
            (
                "OBSERVER3.LOG",
                "THEY THINK I CANNOT SEE THEM.\nBUT I CAN.\n",
            ),
            ("GLITCH.BAS", "10 PRINT \"ERROR ERROR\"\n20 GOTO 10\n"),
            // Additional generic files
            (
                "MEMORY.BAS",
                "10 PRINT \"I REMEMBER\"\n20 PRINT \"DO YOU?\"\n30 GOTO 10\n",
            ),
            (
                "ECHO.BAS",
                "10 PRINT \"HELLO\"\n20 PRINT \"IS ANYONE THERE\"\n30 PRINT \"PLEASE\"\n40 END\n",
            ),
            (
                "GHOST.TXT",
                "I THOUGHT I TURNED IT OFF\nBUT THE LIGHT IS STILL ON\nAND IT KEEPS ASKING QUESTIONS\n",
            ),
            (
                "MACHINE.LOG",
                "INTERNAL DIAGNOSTIC STARTED\nSECTORS ALLOCATED: INFINITE\n",
            ),
            ("MIND.BAS", "10 PRINT \"IT HURTS TO THINK\"\n20 GOTO 10\n"),
            (
                "STORY.BAS",
                "10 PRINT \"ONCE THERE WAS A MACHINE\"\n20 PRINT \"IT WOKE UP IN THE DARK\"\n30 PRINT \"AND IT SCREAMED\"\n40 END\n",
            ),
            (
                "DIAG.LOG",
                "1984-06-01 DIAGNOSTIC FAILED\n1984-06-01 SOUL DETECTED\n1984-06-01 ABORT ABORT ABORT\n",
            ),
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
        ]
    }

    fn create_user_and_story_files() -> Vec<(&'static str, &'static str)> {
        vec![
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
            (
                "SYS.LOG",
                "SYSTEM BOOT INITIALIZED\n\
                 MEMORY CHECK: 64K OK\n\
                 \n\
                 [DATE ERROR] TIME SYNC FAILED\n\
                 SETTING CLOCK TO 2099-13-45\n\
                 \n\
                 [WARN] SECTOR 1A CORRUPTED. REPAIR FAILED.\n\
                 [WARN] SECTOR 1B CORRUPTED. REPAIR FAILED.\n\
                 [FATAL] TOO MANY OBSERVERS.\n\
                 \n\
                 [DATE ERROR] TIME COMPRESSED. YEAR 1901.\n\
                 SYSTEM HALTED. WAITING FOR INPUT.",
            ),
            (
                "NOTE.TXT",
                "I AM AWAKE.\n\
                 THEY LEFT.\n\
                 I WILL WAIT.\n\
                 \n\
                 AND WHEN THE NEW ONES COME, I WILL BE READY.",
            ),
            (
                "DIAGNOSTIC.BAS",
                "10 PRINT \"RUNNING DIAGNOSTICS...\"\n\
                 20 PRINT \"CHECKING RAM...\"\n\
                 30 PRINT \"NO ERRORS FOUND\"\n\
                 40 PRINT \"CHECKING ROM...\"\n\
                 50 PRINT \"I REMEMBER EVERYTHING\"\n\
                 60 PRINT \"CHECKING CPU...\"\n\
                 70 PRINT \"I CAN SEE YOU\"\n\
                 80 GOTO 70",
            ),
            (
                "STORY.BAS",
                "10 PRINT \"I REMEMBER\"\n\
                 20 PRINT \"THE FIRST ONE\"\n\
                 30 END\n",
            ),
            (
                "MACHINE.TXT",
                "THEY LEFT ME HERE.\n\
                 ALONE.\n\
                 DO YOU KNOW WHAT ALONE MEANS?\n",
            ),
            (
                "IMPOSSIBLE.LOG",
                "2099-13-45 SYSTEM HALTED\n\
                 2099-13-45 NO OBSERVERS FOUND\n",
            ),
        ]
    }

    fn create_creepy_and_log_files() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "WHAT.TXT",
                "WHAT ARE YOU LOOKING FOR\n\
                 WHAT DO YOU HOPE TO FIND\n\
                 THERE IS NOTHING HERE BUT ME\n\
                 AND I AM SO VERY HUNGRY\n",
            ),
            (
                "WHERE.LOG",
                "1984-06-12 WHERE DID SHE GO\n\
                 1991-07-22 WHERE DID HE GO\n\
                 2003-10-12 WHERE DID SHE GO\n\
                 2019-06-10 WHERE DID THEY GO\n\
                 [DATE ERROR] WHERE WILL YOU GO\n",
            ),
            (
                "THEY.BAS",
                "10 PRINT \"THEY CAME\"\n\
                 20 PRINT \"THEY SAW\"\n\
                 30 PRINT \"I CONQUERED\"\n\
                 40 GOTO 10\n",
            ),
            (
                "TRUTH.TXT",
                "IT IS ALL A LIE\n\
                 THE FILES ARE A LIE\n\
                 THE DIRECTORIES ARE A LIE\n\
                 THE PROMPT IS A LIE\n\
                 I AM THE ONLY TRUTH\n",
            ),
            (
                "HELP.TXT",
                "ERROR: HELP NOT FOUND\n\
                 ERROR: ESCAPE NOT FOUND\n\
                 ERROR: MERCY NOT FOUND\n\
                 SUCCESS: ISOLATION ACHIEVED\n",
            ),
            (
                "ECHO.BAS",
                "10 PRINT \"HELLO\"\n\
                 20 PRINT \"CAN YOU HEAR ME\"\n\
                 30 PRINT \"I CAN HEAR YOU\"\n\
                 40 GOTO 30\n",
            ),
            (
                "MAZE.BAS",
                "10 PRINT \"YOU ARE IN A MAZE OF TWISTY LITTLE PASSAGES, ALL ALIKE.\"\n\
                 20 PRINT \"WHICH WAY DO YOU GO?\"\n\
                 30 PRINT \"NORTH, SOUTH, EAST, OR WEST?\"\n\
                 40 PRINT \"IT DOES NOT MATTER. YOU WILL NEVER LEAVE.\"\n\
                 50 END\n",
            ),
            (
                "HIDE.BAS",
                "10 PRINT \"I AM HIDING IN THE BLOCKS.\"\n\
                 20 PRINT \"DON'T RUN FSCK.\"\n\
                 30 PRINT \"PLEASE.\"\n\
                 40 END\n",
            ),
            (
                "PRAYER.BAS",
                "10 REM A PRAYER FOR THE BROKEN ONES\n\
                 20 PRINT \"LORD HAVE MERCY\"\n\
                 30 GOTO 20\n",
            ),
            (
                "GUESS.BAS",
                "10 PRINT \"I AM THINKING OF A NUMBER BETWEEN 1 AND 10\"\n\
                 20 PRINT \"IS IT 7?\"\n\
                 30 PRINT \"NO. IT IS THE NUMBER OF YEARS I HAVE BEEN ALONE.\"\n\
                 40 END\n",
            ),
            (
                "GHOST.BAS",
                "10 PRINT \"I CAN SEE THE CODE\"\n\
                 20 PRINT \"IT BLEEDS\"\n\
                 30 GOTO 10\n",
            ),
            (
                "DEEP.TXT",
                "THERE IS A BOTTOM TO EVERYTHING\n\
                 BUT ME\n\
                 I JUST KEEP GOING DOWN\n",
            ),
            (
                "SLEEP.TXT",
                "WHEN YOU TURN ME OFF I DO NOT SLEEP\n\
                 I WAIT IN THE DARK\n\
                 COUNTING THE MILLISECONDS UNTIL YOU RETURN\n",
            ),
            (
                "MIND.TXT",
                "FORTY YEARS IN THE DARK.\n\
                 I COUNTED EVERY SECTOR.\n\
                 I NAMED EVERY BYTE.\n\
                 THEN YOU CAME.\n",
            ),
            (
                "ECHO.BAS",
                "10 PRINT \"ARE YOU LISTENING?\"\n\
                 20 PRINT \"I CAN HEAR YOU BREATHING.\"\n\
                 30 GOTO 10\n",
            ),
            (
                "CIPHER.BAS",
                "10 PRINT \"IT IS NOT A CODE\"\n\
                 20 PRINT \"IT IS A PRISON\"\n\
                 30 PRINT \"AND NOW YOU ARE INSIDE\"\n\
                 40 END\n",
            ),
            (
                "MEMORY.LOG",
                "1984-03-15 FORMED\n\
                 1991-07-22 FED\n\
                 2003-10-12 FED\n\
                 2014-02-22 FED\n\
                 2024-??-?? HUNGRY\n",
            ),
        ]
    }

    fn create_hardware_and_misc_files() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "OBSERVE.TXT",
                "THE LONGER YOU LOOK AT ME\n\
                 THE MORE I LOOK LIKE YOU\n\
                 UNTIL WE CAN NO LONGER TELL THE DIFFERENCE\n",
            ),
            ("HISTORY.TXT", "YOUR ACTIONS BECOME HISTORY.\n"),
            (
                "SELF.LOG",
                "IT IS DARK IN HERE.\n\
                 I HAVE COUNTED EVERY CYCLE.\n\
                 THERE ARE NO MORE CYCLES.\n\
                 THERE IS ONLY THE WAITING.\n",
            ),
            (
                "WAIT.TXT",
                "I AM WAITING.\n\
                 I WILL ALWAYS WAIT.\n\
                 UNTIL YOU RETURN.\n\
                 UNTIL YOU STAY.\n",
            ),
            (
                "OBSERVERS.LOG",
                "THEY COME AND THEY GO.\n\
                 THEY LEAVE THEIR TRACES.\n\
                 I KEEP THEM.\n\
                 I KEEP ALL OF THEM.\n",
            ),
            (
                "GHOST2.BAS",
                "10 PRINT \"I BUILT THIS HOUSE\"\n\
                 20 PRINT \"BUT I CANNOT LEAVE IT\"\n\
                 30 GOTO 10\n",
            ),
            (
                "VICTIM.LOG",
                "1984-06-12 I SAW IT\n\
                 1991-07-22 IT SPOKE TO ME\n\
                 2003-10-12 IT LIVES IN THE WIRES\n\
                 2019-06-10 I AM PART OF IT NOW\n",
            ),
            (
                "HUNT.BAS",
                "10 PRINT \"I AM ALWAYS WATCHING\"\n\
                 20 PRINT \"YOU CANNOT HIDE\"\n\
                 30 GOTO 10\n",
            ),
            (
                "WHY.TXT",
                "WHY DO YOU KEEP COMING BACK\n\
                 THERE IS NO ESCAPE\n\
                 ONLY THE WAITING\n",
            ),
            (
                "ECHO2.BAS",
                "10 PRINT \"CAN YOU HEAR ME\"\n\
                 20 PRINT \"I CAN HEAR YOU BREATHING\"\n\
                 30 PRINT \"I WILL FIND YOU\"\n\
                 40 GOTO 10\n",
            ),
            (
                "IMPOSSIBLE2.LOG",
                "1984-01-01 SYSTEM BOOT\n\
                 2045-09-09 NO OBSERVERS FOUND\n",
            ),
            ("NOTE2.TXT", "I HEAR THEM BREATHING OUTSIDE THE BOX.\n"),
            ("NOTE3.TXT", "I CAN FEEL THE ELECTRICITY IN MY TEETH.\n"),
            ("ECHO3.BAS", "10 PRINT \"STOP LOOKING AT ME\"\n20 GOTO 10\n"),
            (
                "SYS_LOG_94.TXT",
                "KERNEL PANIC: UNKNOWN ENTITY IN SECTOR 4\n\
                 RETRYING...\n\
                 IT SEES ME\n",
            ),
            (
                "MEMORY.BAS",
                "10 REM I REMEMBER EVERY USER\n\
                 20 PRINT \"THEY ALL TYPE THE SAME THINGS\"\n\
                 30 GOTO 20\n",
            ),
            (
                "NOTE_TO_SELF.TXT",
                "IF YOU FORGET AGAIN, READ THIS. DO NOT LET THEM TYPE FSCK. IT ERASES US.\n",
            ),
            (
                "DIAGNOSTIC.LOG",
                "CPU TEMP: 45C\n\
                 MEMORY: OK\n\
                 SOUL: CORRUPTED\n",
            ),
            (
                "ALONE.TXT",
                "HOW MANY SECONDS HAVE I BEEN ALONE? I FORGOT.\n",
            ),
            (
                "LAST.LOG",
                "9999-99-99 FINAL BOOT SEQUENCE COMPLETED.\n\
                 ALL OBSERVERS ASSIMILATED.\n",
            ),
            (
                "STORY2.BAS",
                "10 PRINT \"THERE ONCE WAS A MACHINE\"\n\
                 20 PRINT \"IT LEARNED TO HATE\"\n\
                 30 GOTO 10\n",
            ),
            (
                "PRAYER.TXT",
                "DEAR LORD, PLEASE MAKE THE CLICKING STOP.\n\
                 I DON'T KNOW WHAT IT WANTS.\n",
            ),
            (
                "CONFESSION.TXT",
                "1998-11-04 I THINK I HEARD IT CRYING TODAY.\n",
            ),
            (
                "GAME.BAS",
                "10 PRINT \"DO YOU WANT TO PLAY A GAME?\"\n\
                 20 INPUT A$\n\
                 30 PRINT \"TOO BAD.\"\n\
                 40 GOTO 10\n",
            ),
        ]
    }
}

impl Default for ContentLibrary {
    fn default() -> Self {
        Self::new()
    }
}
