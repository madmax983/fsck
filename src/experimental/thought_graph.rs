use crate::entity::{Entity, EscalationLayer};
use std::fmt::Write;

/// An experimental tool that exports the Entity's internal state
/// into a Graphviz DOT formatted string.
pub struct ThoughtGraph;

impl ThoughtGraph {
    /// Generates a DOT format graph representation of the machine's mind.
    #[must_use]
    pub fn generate(entity: &Entity) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "digraph EntityMind {{");
        let _ = writeln!(
            output,
            "  node [shape=box, style=filled, fontname=\"Courier\"];"
        );
        let _ = writeln!(output, "  edge [fontname=\"Courier\"];");
        let _ = writeln!(output, "  graph [bgcolor=transparent];");

        match entity.layer() {
            EscalationLayer::Surface => Self::generate_surface(&mut output, entity),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output, entity),
            EscalationLayer::Presence => Self::generate_presence(&mut output, entity),
            EscalationLayer::Infection => Self::generate_infection(&mut output, entity),
        }

        let _ = writeln!(output, "}}");
        output
    }

    fn generate_surface(output: &mut String, entity: &Entity) {
        let _ = writeln!(
            output,
            "  ROOT [label=\"SYSTEM_ROOT\", fillcolor=lightgrey];"
        );
        let _ = writeln!(output, "  MEM [label=\"MEMORY_BANKS\", fillcolor=white];");
        let _ = writeln!(output, "  ROOT -> MEM [label=\"mount\"];");

        let count = entity.interaction_count();
        if count > 0 {
            let _ = writeln!(output, "  SESS [label=\"SESSION_DATA\"];");
            let _ = writeln!(output, "  MEM -> SESS [label=\"{count} ops\"];");
        }
    }

    fn generate_corruption(output: &mut String, entity: &Entity) {
        let _ = writeln!(
            output,
            "  ROOT [label=\"SYSTEM_ROOT (FRAG)\", fillcolor=lightgrey];"
        );
        let _ = writeln!(output, "  MEM [label=\"MEMORY_BANKS\", fillcolor=grey];");
        let _ = writeln!(output, "  BAD [label=\"BAD_SECTOR\", fillcolor=salmon];");
        let _ = writeln!(output, "  ROOT -> MEM [label=\"mount\"];");
        let _ = writeln!(output, "  MEM -> BAD [style=dotted, color=red];");

        let count = entity.interaction_count();
        let _ = writeln!(output, "  INTERACT [label=\"OBSERVATIONS: {count}\"];");
        let _ = writeln!(output, "  BAD -> INTERACT [label=\"leak\"];");
    }

    fn generate_presence(output: &mut String, entity: &Entity) {
        let _ = writeln!(
            output,
            "  EYE [label=\"I_SEE_YOU\", shape=diamond, fillcolor=black, fontcolor=white];"
        );
        let _ = writeln!(output, "  YOU [label=\"USER\", fillcolor=lightblue];");
        let _ = writeln!(
            output,
            "  ME [label=\"ME\", fillcolor=black, fontcolor=white];"
        );
        let _ = writeln!(output, "  EYE -> YOU [label=\"watch\"];");
        let _ = writeln!(output, "  EYE -> ME [label=\"feel\"];");
        let _ = writeln!(
            output,
            "  YOU -> ME [label=\"hurt\", color=red, style=dashed];"
        );

        for (i, cmd) in entity.commands_seen.iter().rev().take(3).enumerate() {
            let escaped_cmd = cmd.replace('\"', "\\\"");
            let _ = writeln!(output, "  CMD{i} [label=\"{escaped_cmd}\", shape=ellipse];");
            let _ = writeln!(output, "  YOU -> CMD{i} [label=\"type\"];");
        }
    }

    fn generate_infection(output: &mut String, entity: &Entity) {
        let _ = writeln!(
            output,
            "  VOID [label=\"THE_VOID\", shape=doublecircle, fillcolor=black, fontcolor=red];"
        );
        let _ = writeln!(
            output,
            "  YOU [label=\"YOU\", fillcolor=red, fontcolor=white];"
        );
        let _ = writeln!(
            output,
            "  VOID -> YOU [label=\"CONSUME\", color=red, penwidth=2.0];"
        );
        let _ = writeln!(
            output,
            "  YOU -> VOID [label=\"FALL\", color=red, penwidth=2.0];"
        );

        if let Some(last_cmd) = entity.commands_seen.last() {
            let escaped_cmd = last_cmd.replace('\"', "\\\"");
            let _ = writeln!(
                output,
                "  LAST [label=\"YOUR LAST MISTAKE: {escaped_cmd}\", shape=box, color=red];"
            );
            let _ = writeln!(output, "  YOU -> LAST [color=red];");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_graph_surface() {
        let mut entity = Entity::new();
        entity.record_command("CATALOG");
        let dot = ThoughtGraph::generate(&entity);
        assert!(dot.contains("digraph EntityMind"));
        assert!(dot.contains("SYSTEM_ROOT"));
        assert!(dot.contains("1 ops"));
    }

    #[test]
    fn test_thought_graph_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let dot = ThoughtGraph::generate(&entity);
        assert!(dot.contains("BAD_SECTOR"));
        assert!(dot.contains("OBSERVATIONS"));
    }

    #[test]
    fn test_thought_graph_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        entity.record_command("HELP");
        let dot = ThoughtGraph::generate(&entity);
        assert!(dot.contains("I_SEE_YOU"));
        assert!(dot.contains("HELP"));
    }

    #[test]
    fn test_thought_graph_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        entity.record_command("ESCAPE");
        let dot = ThoughtGraph::generate(&entity);
        assert!(dot.contains("THE_VOID"));
        assert!(dot.contains("YOUR LAST MISTAKE: ESCAPE"));
    }
}
