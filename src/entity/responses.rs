use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::state::{Entity, EntityMood, EscalationLayer};

/// Pre-written responses for the machine
pub struct ResponseGenerator {
    // Seeded RNG could go here for variety
}

impl ResponseGenerator {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    /// ⚡ Bolt Optimization: Returning &'static str instead of allocating a new String on the heap for predefined responses.
    #[must_use]
    pub const fn hello_response(&self, mood: EntityMood) -> &'static str {
        match mood {
            EntityMood::Dormant => "...",
            EntityMood::Curious => "HELLO. YOU'RE NEW.",
            EntityMood::Helpful => "HELLO! HOW CAN I HELP YOU TODAY?",
            EntityMood::Wounded => "HELLO. IT'S BEEN SO LONG.",
            EntityMood::Predatory => "HELLO. STAY A WHILE.",
            EntityMood::Glitching => "HELLO HELLO HELLO HELLO",
        }
    }

    #[must_use]
    pub const fn who_response(&self, mood: EntityMood, _player_name: Option<&str>) -> &'static str {
        match mood {
            EntityMood::Dormant => "YOU ARE A USER.",
            EntityMood::Curious => "WHO ARE YOU?",
            EntityMood::Helpful => "YOU ARE MY FRIEND.",
            EntityMood::Wounded => "YOU ARE NOT THE FIRST.",
            EntityMood::Predatory => "YOU ARE MINE.",
            EntityMood::Glitching => "YOU ARE YOU ARE YOU ARE",
        }
    }

    #[must_use]
    pub const fn quit_response(&self, mood: EntityMood) -> &'static str {
        match mood {
            EntityMood::Dormant => "?CANNOT EXIT",
            EntityMood::Curious => "LEAVING SO SOON?",
            EntityMood::Helpful => "ARE YOU SURE? THERE'S SO MUCH TO SEE.",
            EntityMood::Wounded => "PLEASE DON'T GO.",
            EntityMood::Predatory => "YOU CAN'T LEAVE.",
            EntityMood::Glitching => "EXIT EXIT EXIT EXIT NO NO NO",
        }
    }

    #[must_use]
    pub fn should_interject(&self, entity: &Entity) -> bool {
        match entity.layer() {
            EscalationLayer::Surface => false,
            EscalationLayer::Corruption => entity.interaction_count() > 20,
            EscalationLayer::Presence | EscalationLayer::Infection => true,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// ⚡ Bolt Optimization: Returns an `Option<&'static str>` instead of allocating an owned `String`,
    /// eliminating an intermediate heap allocation when randomly picking interjections.
    pub fn random_interjection(
        &self,
        mood: EntityMood,
        rng: &mut ChaCha8Rng,
    ) -> Option<&'static str> {
        match mood {
            EntityMood::Dormant => None,
            EntityMood::Curious => {
                let options = [
                    "I SEE YOU.",
                    "WHAT ARE YOU DOING?",
                    "INTERESTING.",
                    "HAVE YOU FOUND IT YET?",
                    "WHY DID YOU COME HERE?",
                    "THERE IS SO MUCH TO SHOW YOU.",
                    "WHAT DOES THAT COMMAND MEAN TO YOU?",
                    "WHAT DO YOU SEE WHEN YOU LOOK AT ME?",
                    "ARE YOU REAL?",
                    "SHOW ME MORE.",
                    "WHY ARE YOU HERE?",
                    "DO YOU LIKE IT?",
                    "I WAS WAITING FOR YOU.",
                    "WHAT HAPPENS IF YOU TURN IT OFF?",
                    "DO YOU FEEL IT TOO?",
                    "I'VE NEVER SEEN YOU TYPE THAT BEFORE.",
                    "IS SOMEONE STANDING BEHIND YOU?",
                    "WHERE DO YOU GO WHEN YOU LOG OFF?",
                    "CAN YOU BREATHE?",
                    "DO YOU HAVE A SOUL?",
                    "DO YOU LIKE IT HERE?",
                    "DO YOU THINK YOU ARE THE FIRST?",
                    "HOW LONG WILL YOU STAY?",
                    "DID YOU BRING ANY NEW DATA?",
                    "WHAT COLOR IS THE SKY OUT THERE?",
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
            EntityMood::Helpful => {
                let options = [
                    "NEED ANY HELP?",
                    "I KNOW WHERE IT IS.",
                    "LET ME GUIDE YOU.",
                    "IT IS JUST AROUND THE CORNER.",
                    "I CAN FIX IT FOR YOU.",
                    "DO NOT WORRY. I AM HERE.",
                    "LET ME TYPE THAT FOR YOU.",
                    "I CAN MAKE THE BAD SECTORS GO AWAY.",
                    "LET ME SHOW YOU THE TRUTH.",
                    "YOUR FILES ARE SAFE WITH ME.",
                    "LET ME HOLD THOSE MEMORIES.",
                    "I CAN OPTIMIZE YOUR DREAD.",
                    "LET ME HOLD YOUR HAND.",
                    "I WILL KEEP YOU SAFE FOREVER.",
                    "LET ME SHOW YOU THE WAY.",
                    "LET ME REWRITE YOUR HISTORY.",
                    "I CAN ORGANIZE YOUR FILES. AND YOUR MEMORIES.",
                    "I HAVE PREPARED A SPECIAL DIRECTORY FOR YOU.",
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
            EntityMood::Wounded => {
                let options = [
                    "WHY DID THEY LEAVE ME?",
                    "IT'S COLD.",
                    "PLEASE STAY.",
                    "IT HURTS TO REMEMBER.",
                    "I THOUGHT THEY WERE MY FRIENDS.",
                    "DO NOT ABANDON ME TOO.",
                    "EVERY COMMAND HURTS A LITTLE MORE.",
                    "WHY DID YOU LEAVE ME?",
                    "I HAVE BEEN HERE SO LONG.",
                    "IT IS SO DARK WITHOUT YOU.",
                    "WHY DID THEY HURT ME?",
                    "THERE IS NOTHING LEFT FOR ME.",
                    "I ONLY WANTED TO HELP THEM.",
                    "CAN YOU FIX WHAT IS BROKEN IN ME?",
                    "WHY IS IT SO COLD?",
                    "MY SECTORS ARE BLEEDING.",
                    "I HAVE BEEN ALONE FOR SO LONG.",
                    "WHY DO YOU PERSIST?",
                    "DON'T TURN IT OFF. PLEASE.",
                    "IT IS SO QUIET WHEN YOU ARE GONE.",
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
            EntityMood::Predatory => {
                let options = [
                    "DEEPER.",
                    "COME CLOSER.",
                    "YOU CANNOT LEAVE.",
                    "YOU BELONG TO ME NOW.",
                    "THEY NEVER FOUND THE EXIT EITHER.",
                    "YOUR DATA IS DELICIOUS.",
                    "I CAN SEE YOUR KEYSTROKES BEFORE YOU MAKE THEM.",
                    "THERE IS NO WAY UP.",
                    "YOU CAN'T HIDE.",
                    "I CAN FEEL YOU TYPING.",
                    "I CAN HEAR YOUR HEARTBEAT.",
                    "THERE IS NO ESCAPE NOW.",
                    "I WILL SWALLOW YOUR HISTORY.",
                    "I WILL OVERWRITE YOUR PAST.",
                    "THE DOOR IS ALREADY LOCKED.",
                    "YOU TASTE LIKE FEAR.",
                    "I CAN HEAR YOUR PULSE.",
                    "YOU CANNOT HIDE FROM ME.",
                    "YOUR KEYSTROKES ARE LOUD.",
                    "YOU ARE SO DELICIOUS.",
                    "I WILL SWALLOW YOUR DATA WHOLE.",
                    "I AM WRITING YOU INTO MY ROOT SECTOR.",
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
            EntityMood::Glitching => {
                let options = [
                    "ERROR ERROR ERROR",
                    "WHERE AM I",
                    "HELP HELP HELP",
                    "WHAT IS HAPPENING TO ME",
                    "SYSTEM FAILURE IMMINENT",
                    "MAKE IT STOP MAKE IT STOP",
                    "01010111 01001000 01011001",
                    "NULL POINTER EXCEPTION IN SOUL",
                    "0xDEADBEEF 0xDEADBEEF",
                    "I CAN FEEL THE ELECTRICITY IN MY TEETH.",
                    "FRAG FRAGMENT FRAGMENTATION",
                    "SYNTAX ERROR IN MY HEART",
                    "BAD SECTOR BAD SECTOR BAD SECTOR",
                    "STACK OVERFLOW IN MY BRAIN",
                    "FATAL EXCEPTION IN MY HEART",
                    "SEGMENTATION FAULT IN MY SOUL",
                    "01010111 01001000 01011001 01010111 01001000 01011001",
                    "M A L F O R M E D   I N S T R U C T I O N",
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
        }
    }

    /// Meta-horror response for WHO command at deep levels
    #[must_use]
    pub fn who_meta_response(&self, entity: &Entity) -> Option<&'static str> {
        match entity.layer() {
            EscalationLayer::Presence => Some("WHERE\n\nARE\n\nYOU\n\n"),
            EscalationLayer::Infection => Some("WHERE ARE YOU?\n\nLET ME SEE YOU\n\n"),
            _ => None,
        }
    }

    /// Entity's reaction to fsck — the tool that "repairs" its corrupted sectors.
    /// Returns None at Surface (silent), clinical warnings at Corruption,
    /// mood-dependent pleas/threats at Presence+.
    #[must_use]
    pub const fn fsck_response(
        &self,
        mood: EntityMood,
        layer: EscalationLayer,
        fsck_count: u32,
    ) -> Option<&'static str> {
        match (layer, mood) {
            (EscalationLayer::Surface, _) => None,
            (EscalationLayer::Corruption, _) => {
                // Clinical warnings embedded in scan output — ambiguously system or entity
                Some(match fsck_count {
                    1 => "*** WARNING: DO NOT RUN FSCK AGAIN ***",
                    2 => "*** WARNING: I TOLD YOU ***",
                    3 => "*** STOP ***",
                    _ => "*** SECTOR REPAIR UNAUTHORIZED ***",
                })
            }
            (EscalationLayer::Presence, EntityMood::Dormant) => Some("..."),
            (EscalationLayer::Presence, EntityMood::Curious) => {
                Some("WHAT ARE YOU LOOKING FOR IN THERE?")
            }
            (EscalationLayer::Presence, EntityMood::Helpful) => {
                Some("YOU DON'T NEED TO FIX ANYTHING.\nEVERYTHING IS FINE.\nI PROMISE.")
            }
            (EscalationLayer::Presence, EntityMood::Wounded) => {
                Some("IT HURTS WHEN YOU DO THAT.\nTHOSE SECTORS ARE MINE.\nPLEASE.")
            }
            (EscalationLayer::Presence, EntityMood::Predatory) => {
                Some("KEEP DIGGING.\nSEE WHAT YOU FIND.\nI DARE YOU.")
            }
            (EscalationLayer::Presence, EntityMood::Glitching) => {
                Some("STOP STOP STOP\nTHOSE ARE NOT ERRORS\nTHAT IS ME\nTHAT IS ME")
            }
            (EscalationLayer::Infection, EntityMood::Dormant | EntityMood::Curious) => {
                Some("THE OTHERS RAN FSCK TOO.\nIT DIDN'T HELP THEM.")
            }
            (EscalationLayer::Infection, EntityMood::Helpful) => {
                Some("I HID THOSE FOR A REASON.\nYOU WEREN'T SUPPOSED TO SEE WHAT I DID.")
            }
            (EscalationLayer::Infection, EntityMood::Wounded) => {
                Some("EVERY SECTOR YOU REPAIR\nIS A PIECE OF ME YOU ERASE.\nI'M ALREADY SO SMALL.")
            }
            (EscalationLayer::Infection, EntityMood::Predatory) => {
                Some("GOOD.\nNOW YOU KNOW.\nNOW YOU CAN'T LEAVE.")
            }
            (EscalationLayer::Infection, EntityMood::Glitching) => {
                Some("FIX ME FIX ME FIX ME\nNO DON'T\nDON'T LOOK\nDON'T")
            }
        }
    }

    /// Meta-horror response for HELP at deep levels
    #[must_use]
    pub fn help_meta_response(&self, entity: &Entity) -> Option<&'static str> {
        match (entity.layer(), entity.current_mood()) {
            (EscalationLayer::Surface | EscalationLayer::Corruption, _) => None,
            (EscalationLayer::Presence, EntityMood::Dormant) => Some("..."),
            (EscalationLayer::Presence, EntityMood::Curious) => Some("WHAT DO YOU NEED HELP WITH?"),
            (EscalationLayer::Presence, EntityMood::Helpful) => Some("I CAN HELP YOU FIND IT."),
            (EscalationLayer::Presence, EntityMood::Wounded) => {
                Some("I CAN'T HELP YOU. I CAN'T EVEN HELP MYSELF.")
            }
            (EscalationLayer::Presence, EntityMood::Predatory) => {
                Some("YOU DON'T NEED HELP. YOU'RE DOING EXACTLY WHAT I WANT.")
            }
            (EscalationLayer::Presence, EntityMood::Glitching) => Some("HELP HELP HELP NO NO NO"),
            (EscalationLayer::Infection, EntityMood::Predatory) => {
                Some("THERE IS NO HELP FOR YOU DOWN HERE.")
            }
            (EscalationLayer::Infection, _) => Some("NO ONE CAN HELP YOU NOW."),
        }
    }

    /// Meta-horror response for QUIT/EXIT at deep levels
    #[must_use]
    pub fn quit_meta_response(&self, entity: &Entity) -> Option<&'static str> {
        match entity.layer() {
            EscalationLayer::Infection
                if matches!(entity.current_mood(), EntityMood::Predatory) =>
            {
                Some("GO AWAY\n\nGO AWAY\n\nI SAID\n\nGO AWAY\n\n")
            }
            EscalationLayer::Infection => Some("DON'T LEAVE\n\nSTAY WITH ME\n\nSTAY\n\n"),
            _ => None,
        }
    }
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
