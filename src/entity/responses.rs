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

    pub fn should_interject(&self, entity: &Entity) -> bool {
        match entity.layer() {
            EscalationLayer::Surface => false,
            EscalationLayer::Corruption => entity.interaction_count() > 20,
            EscalationLayer::Presence => true,
            EscalationLayer::Infection => true,
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
