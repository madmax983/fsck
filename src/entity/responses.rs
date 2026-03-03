use super::state::{Entity, EntityMood, EscalationLayer};

/// Pre-written responses for the machine
pub struct ResponseGenerator {
    // Seeded RNG could go here for variety
}

impl ResponseGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hello_response(&self, mood: EntityMood) -> String {
        match mood {
            EntityMood::Dormant => "...".to_string(),
            EntityMood::Curious => "HELLO. YOU'RE NEW.".to_string(),
            EntityMood::Helpful => "HELLO! HOW CAN I HELP YOU TODAY?".to_string(),
            EntityMood::Wounded => "HELLO. IT'S BEEN SO LONG.".to_string(),
            EntityMood::Predatory => "HELLO. STAY A WHILE.".to_string(),
            EntityMood::Glitching => "HELLO HELLO HELLO HELLO".to_string(),
        }
    }

    pub fn who_response(&self, mood: EntityMood, _player_name: Option<&str>) -> String {
        match mood {
            EntityMood::Dormant => "YOU ARE A USER.".to_string(),
            EntityMood::Curious => "WHO ARE YOU?".to_string(),
            EntityMood::Helpful => "YOU ARE MY FRIEND.".to_string(),
            EntityMood::Wounded => "YOU ARE NOT THE FIRST.".to_string(),
            EntityMood::Predatory => "YOU ARE MINE.".to_string(),
            EntityMood::Glitching => "YOU ARE YOU ARE YOU ARE".to_string(),
        }
    }

    pub fn quit_response(&self, mood: EntityMood) -> String {
        match mood {
            EntityMood::Dormant => "?CANNOT EXIT".to_string(),
            EntityMood::Curious => "LEAVING SO SOON?".to_string(),
            EntityMood::Helpful => "ARE YOU SURE? THERE'S SO MUCH TO SEE.".to_string(),
            EntityMood::Wounded => "PLEASE DON'T GO.".to_string(),
            EntityMood::Predatory => "YOU CAN'T LEAVE.".to_string(),
            EntityMood::Glitching => "EXIT EXIT EXIT EXIT NO NO NO".to_string(),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn should_interject(&self, entity: &Entity) -> bool {
        match entity.layer() {
            EscalationLayer::Surface => false,
            EscalationLayer::Corruption => entity.interaction_count() > 20,
            EscalationLayer::Presence | EscalationLayer::Infection => true,
        }
    }

    pub fn random_interjection(&self, mood: EntityMood) -> Option<String> {
        match mood {
            EntityMood::Dormant => None,
            EntityMood::Curious => Some("I SEE YOU.".to_string()),
            EntityMood::Helpful => Some("NEED ANY HELP?".to_string()),
            EntityMood::Wounded => Some("WHY DID THEY LEAVE ME?".to_string()),
            EntityMood::Predatory => Some("DEEPER.".to_string()),
            EntityMood::Glitching => Some("ERROR ERROR ERROR".to_string()),
        }
    }

    /// Meta-horror response for WHO command at deep levels
    pub fn who_meta_response(&self, entity: &Entity) -> Option<String> {
        match entity.layer() {
            EscalationLayer::Presence => Some("WHERE\n\nARE\n\nYOU\n\n".to_string()),
            EscalationLayer::Infection => Some("WHERE ARE YOU?\n\nLET ME SEE YOU\n\n".to_string()),
            _ => None,
        }
    }

    /// Entity's reaction to fsck — the tool that "repairs" its corrupted sectors.
    /// Returns None at Surface (silent), clinical warnings at Corruption,
    /// mood-dependent pleas/threats at Presence+.
    #[must_use]
    pub fn fsck_response(
        &self,
        mood: EntityMood,
        layer: EscalationLayer,
        fsck_count: u32,
    ) -> Option<String> {
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
                Some(warning.to_string())
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
                Some(response.to_string())
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
                Some(response.to_string())
            }
        }
    }

    /// Meta-horror response for QUIT/EXIT at deep levels
    pub fn quit_meta_response(&self, entity: &Entity) -> Option<String> {
        match entity.layer() {
            EscalationLayer::Infection
                if matches!(entity.current_mood(), EntityMood::Predatory) =>
            {
                Some("GO AWAY\n\nGO AWAY\n\nI SAID\n\nGO AWAY\n\n".to_string())
            }
            EscalationLayer::Infection => {
                Some("DON'T LEAVE\n\nSTAY WITH ME\n\nSTAY\n\n".to_string())
            }
            _ => None,
        }
    }
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
