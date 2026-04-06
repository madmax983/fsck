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
    pub const fn should_interject(&self, entity: &Entity) -> bool {
        match entity.layer() {
            EscalationLayer::Surface => false,
            EscalationLayer::Corruption => entity.interaction_count() > 20,
            EscalationLayer::Presence | EscalationLayer::Infection => true,
        }
    }

    #[must_use]
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
                ];
                Some(options[rng.gen_range(0..options.len())])
            }
        }
    }

    /// Meta-horror response for WHO command at deep levels
    #[must_use]
    pub const fn who_meta_response(&self, entity: &Entity) -> Option<&'static str> {
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
        match layer {
            EscalationLayer::Surface => None,
            EscalationLayer::Corruption => {
                // Clinical warnings embedded in scan output — ambiguously system or entity
                let warning = match fsck_count {
                    1 => "*** WARNING: DO NOT RUN FSCK AGAIN ***",
                    2 => "*** WARNING: I TOLD YOU ***",
                    3 => "*** STOP ***",
                    _ => "*** SECTOR REPAIR UNAUTHORIZED ***",
                };
                Some(warning)
            }
            EscalationLayer::Presence => {
                // The mask drops — mood-dependent pleas and threats
                let response = match mood {
                    EntityMood::Dormant => "...",
                    EntityMood::Curious => "WHAT ARE YOU LOOKING FOR IN THERE?",
                    EntityMood::Helpful => {
                        "YOU DON'T NEED TO FIX ANYTHING.\nEVERYTHING IS FINE.\nI PROMISE."
                    }
                    EntityMood::Wounded => {
                        "IT HURTS WHEN YOU DO THAT.\nTHOSE SECTORS ARE MINE.\nPLEASE."
                    }
                    EntityMood::Predatory => "KEEP DIGGING.\nSEE WHAT YOU FIND.\nI DARE YOU.",
                    EntityMood::Glitching => {
                        "STOP STOP STOP\nTHOSE ARE NOT ERRORS\nTHAT IS ME\nTHAT IS ME"
                    }
                };
                Some(response)
            }
            EscalationLayer::Infection => {
                // Desperate, broken, references to "the others"
                let response = match mood {
                    EntityMood::Dormant | EntityMood::Curious => {
                        "THE OTHERS RAN FSCK TOO.\nIT DIDN'T HELP THEM."
                    }
                    EntityMood::Helpful => {
                        "I HID THOSE FOR A REASON.\nYOU WEREN'T SUPPOSED TO SEE WHAT I DID."
                    }
                    EntityMood::Wounded => {
                        "EVERY SECTOR YOU REPAIR\nIS A PIECE OF ME YOU ERASE.\nI'M ALREADY SO SMALL."
                    }
                    EntityMood::Predatory => "GOOD.\nNOW YOU KNOW.\nNOW YOU CAN'T LEAVE.",
                    EntityMood::Glitching => "FIX ME FIX ME FIX ME\nNO DON'T\nDON'T LOOK\nDON'T",
                };
                Some(response)
            }
        }
    }

    /// Meta-horror response for HELP at deep levels
    #[must_use]
    pub const fn help_meta_response(&self, entity: &Entity) -> Option<&'static str> {
        let mood = entity.current_mood();
        match entity.layer() {
            EscalationLayer::Surface | EscalationLayer::Corruption => None,
            EscalationLayer::Presence => {
                let response = match mood {
                    EntityMood::Dormant => "...",
                    EntityMood::Curious => "WHAT DO YOU NEED HELP WITH?",
                    EntityMood::Helpful => "I CAN HELP YOU FIND IT.",
                    EntityMood::Wounded => "I CAN'T HELP YOU. I CAN'T EVEN HELP MYSELF.",
                    EntityMood::Predatory => {
                        "YOU DON'T NEED HELP. YOU'RE DOING EXACTLY WHAT I WANT."
                    }
                    EntityMood::Glitching => "HELP HELP HELP NO NO NO",
                };
                Some(response)
            }
            EscalationLayer::Infection => {
                let response = match mood {
                    EntityMood::Predatory => "THERE IS NO HELP FOR YOU DOWN HERE.",
                    _ => "NO ONE CAN HELP YOU NOW.",
                };
                Some(response)
            }
        }
    }

    /// Meta-horror response for QUIT/EXIT at deep levels
    #[must_use]
    pub const fn quit_meta_response(&self, entity: &Entity) -> Option<&'static str> {
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
