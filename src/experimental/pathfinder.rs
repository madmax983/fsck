use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;
use petgraph::algo::dijkstra;
use std::fmt::Write;

pub struct Pathfinder;

impl Pathfinder {
    #[must_use]
    pub fn navigate(fs: &FilesystemGraph, entity: &Entity, target: &str) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "CALCULATING ROUTE TO '{target}'...");

        let graph = fs.graph();
        let start = fs.current_index();

        let target_node = graph.node_indices().find(|&i| graph[i].name().eq_ignore_ascii_case(target));

        let Some(target_idx) = target_node else {
            let _ = writeln!(output, "DESTINATION UNKNOWN.");
            return output;
        };

        let path_map = dijkstra(graph, start, Some(target_idx), |_| 1);

        if !path_map.contains_key(&target_idx) {
            let _ = writeln!(output, "NO PATH FOUND. THE WAY IS BLOCKED.");
            return output;
        }

        let layer = entity.layer();
        let cost = path_map[&target_idx];

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "DISTANCE: {cost} HOPS.");
            },
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "DISTANCE: {cost} HOPS. WATCH YOUR STEP.");
            },
            EscalationLayer::Presence => {
                let _ = writeln!(output, "DISTANCE: {cost} HOPS. IT KNOWS WHERE YOU ARE GOING.");
            },
            EscalationLayer::Infection => {
                let _ = writeln!(output, "DISTANCE: {cost} HOPS. ALL PATHS LEAD DOWN.");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::filesystem::FilesystemGraph;

    #[test]
    fn test_surface_path() {
        let mut fs = FilesystemGraph::new();
        let usr = fs.add_child("USR");
        fs.add_paradox_link(fs.current_index(), usr);
        let entity = Entity::new();
        let result = Pathfinder::navigate(&fs, &entity, "USR");
        assert!(result.contains("DISTANCE:"));
    }
}
