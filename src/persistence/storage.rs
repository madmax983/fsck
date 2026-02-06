#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Storage keys for the game
#[derive(Debug, Clone, Copy)]
pub enum StorageKey {
    GameState,
    EntityMemory,
    PlayerHistory,
}

impl StorageKey {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GameState => "fsck_game_state",
            Self::EntityMemory => "fsck_entity_memory",
            Self::PlayerHistory => "fsck_player_history",
        }
    }
}

/// Wrapper around `LocalStorage`
pub struct GameStorage;

impl GameStorage {
    #[cfg(target_arch = "wasm32")]
    pub fn save(key: StorageKey, value: &str) -> Result<(), String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .set_item(key.as_str(), value)
            .map_err(|_| "Failed to save to localStorage".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load(key: StorageKey) -> Result<Option<String>, String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .get_item(key.as_str())
            .map_err(|_| "Failed to load from localStorage".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn remove(key: StorageKey) -> Result<(), String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .remove_item(key.as_str())
            .map_err(|_| "Failed to remove from localStorage".to_string())
    }

    // Non-WASM stubs for testing

    /// # Errors
    /// Never returns an error in non-WASM builds
    #[cfg(not(target_arch = "wasm32"))]
    pub const fn save(_key: StorageKey, _value: &str) -> Result<(), String> {
        Ok(())
    }

    /// # Errors
    /// Never returns an error in non-WASM builds
    #[cfg(not(target_arch = "wasm32"))]
    pub const fn load(_key: StorageKey) -> Result<Option<String>, String> {
        Ok(None)
    }

    /// # Errors
    /// Never returns an error in non-WASM builds
    #[cfg(not(target_arch = "wasm32"))]
    pub const fn remove(_key: StorageKey) -> Result<(), String> {
        Ok(())
    }
}
