use crate::effects::MetadataCorruptor;
use crate::entity::Entity;
use std::fmt::Write;

pub struct StatTool;

impl StatTool {
    #[must_use]
    pub fn generate_stat(filename: &str, content_len: usize, entity: &Entity, seed: u64) -> String {
        let corruptor = MetadataCorruptor::new(entity.max_depth_reached());

        let ctime = corruptor.corrupt_timestamp("1984-01-24");
        let mtime = corruptor.corrupt_timestamp("1984-01-24");

        let corrupted_filename = corruptor.corrupt_filename(filename, seed);

        let mut output = String::with_capacity(256);

        let _ = writeln!(output, "  File: {corrupted_filename}");
        let _ = writeln!(output, "  Size: {content_len} bytes");
        let _ = writeln!(output, "Access: (0644/-rw-r--r--)");
        let _ = writeln!(output, "Birth:  {ctime}");
        let _ = writeln!(output, "Modify: {mtime}");

        if entity.max_depth_reached() >= 16 {
            let _ = writeln!(output, "Owner:  UNKNOWN");
        } else {
            let _ = writeln!(output, "Owner:  SYSTEM");
        }

        output
    }
}
