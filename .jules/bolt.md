**[Serialization of VecDeque vs Vec]
**Learning:** While VecDeque is O(1) for pop_front(), changing a serialized Vec to VecDeque in a struct like GameState could break backward compatibility for existing user save data.
**Action:** Optimize Vec operations around the edges (like removing unnecessary clones) rather than changing the fundamental data structure if it's tied to persistent storage.
