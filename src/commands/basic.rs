use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::types::CommandResult;
use crate::effects::{CorruptionEffect, CorruptionIntensity};
use crate::entity::{Entity, EscalationLayer, ResponseGenerator};

pub struct BasicEvaluationContext<'a> {
    pub program: &'a std::collections::BTreeMap<u32, String>,
    pub output: &'a mut String,
    pub layer: EscalationLayer,
    pub rng: &'a mut ChaCha8Rng,
    pub next_line: &'a mut Option<u32>,
}

pub struct BasicExecutor;

impl BasicExecutor {
    pub fn parse_basic_program(
        content: &str,
    ) -> Result<std::collections::BTreeMap<u32, String>, String> {
        content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (num_str, stmt) = line.split_once(' ').unwrap_or((line, ""));
                num_str
                    .parse::<u32>()
                    .map(|line_num| (line_num, stmt.trim().to_string()))
                    .map_err(|_| format!("?SYNTAX ERROR IN: {line}\n"))
            })
            .collect()
    }

    fn execute_print_statement(
        stmt: &str,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
        entity: &Entity,
        responses: &ResponseGenerator,
    ) -> String {
        let is_deep_layer = matches!(
            layer,
            EscalationLayer::Corruption | EscalationLayer::Presence | EscalationLayer::Infection
        );

        let possible_interjection = if is_deep_layer && rng.gen_bool(0.15) {
            responses.random_interjection(entity.current_mood(), rng)
        } else {
            None
        };

        if let Some(interjection) = possible_interjection {
            return interjection.to_string();
        }

        let content = stmt.trim_start_matches("PRINT").trim();
        if content.starts_with('"') && content.ends_with('"') && content.len() >= 2 {
            content[1..content.len() - 1].to_string()
        } else {
            content.to_string()
        }
    }

    fn evaluate_goto_statement(
        ctx: &mut BasicEvaluationContext<'_>,
        stmt: &str,
        line_num: u32,
    ) -> Result<bool, CommandResult> {
        let target_str = stmt.trim_start_matches("GOTO").trim();
        let Ok(target) = target_str.parse::<u32>() else {
            use std::fmt::Write;
            let _ = writeln!(ctx.output, "?SYNTAX ERROR IN {line_num}");
            // ⚡ Bolt Optimization: Take the constructed `String` instead of allocating an intermediate formatted `String`.
            let out = std::mem::take(ctx.output);
            return Err(CommandResult::error(out));
        };

        if !ctx.program.contains_key(&target) {
            use std::fmt::Write;
            let _ = writeln!(ctx.output, "?UNDEF'D STATEMENT ERROR IN {line_num}");
            // ⚡ Bolt Optimization: Take the constructed `String` instead of allocating an intermediate formatted `String`.
            let out = std::mem::take(ctx.output);
            return Err(CommandResult::error(out));
        }

        *ctx.next_line = Some(target);
        Ok(true)
    }

    fn evaluate_basic_statement(
        ctx: &mut BasicEvaluationContext<'_>,
        stmt: &str,
        line_num: u32,
        entity: &Entity,
        responses: &ResponseGenerator,
    ) -> Result<bool, CommandResult> {
        if stmt.starts_with("PRINT") {
            let display_text = Self::execute_print_statement(stmt, ctx.layer, ctx.rng, entity, responses);
            ctx.output.push_str(&display_text);
            ctx.output.push('\n');
            return Ok(true);
        }

        if stmt.starts_with("GOTO") {
            return Self::evaluate_goto_statement(ctx, stmt, line_num);
        }

        if stmt.starts_with("END") {
            return Ok(false);
        }

        if stmt.starts_with("REM") {
            return Ok(true);
        }

        if !stmt.is_empty() {
            use std::fmt::Write;
            let _ = writeln!(ctx.output, "?SYNTAX ERROR IN {line_num}");
            // ⚡ Bolt Optimization: Take the constructed `String` instead of allocating an intermediate formatted `String`.
            let out = std::mem::take(ctx.output);
            return Err(CommandResult::error(out));
        }

        Ok(true)
    }

    pub fn execute_basic_program(
        program: &std::collections::BTreeMap<u32, String>,
        entity: &Entity,
        responses: &ResponseGenerator,
    ) -> CommandResult {
        if program.is_empty() {
            return CommandResult::success("");
        }

        // ⚡ Bolt Optimization: Pre-allocate a reasonable capacity for BASIC program output.
        let mut output = String::with_capacity(128);
        let mut iterations = 0;
        let mut current_line = program.keys().next().copied();

        let layer = entity.layer();
        let mut rng = ChaCha8Rng::seed_from_u64(
            0xF5C0_0000u64.wrapping_add(u64::from(entity.interaction_count())),
        );

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.2)
        {
            return CommandResult::error("?CANNOT EXECUTE. IT IS WATCHING.\n");
        }

        while let Some(line_num) = current_line {
            if iterations >= 100 {
                use std::fmt::Write;
                let _ = writeln!(&mut output, "?OUT OF MEMORY ERROR IN {line_num}");
                return CommandResult::error(output);
            }
            iterations += 1;

            let stmt = &program[&line_num];
            let mut next_line = program.range((line_num + 1)..).next().map(|(k, _)| *k);

            let mut ctx = BasicEvaluationContext {
                program,
                output: &mut output,
                layer,
                rng: &mut rng,
                next_line: &mut next_line,
            };

            match Self::evaluate_basic_statement(&mut ctx, stmt, line_num, entity, responses) {
                Ok(true) => current_line = next_line,
                Ok(false) => break,
                Err(e) => return e,
            }
        }

        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(
                &output,
                0xF5C0_0000u64.wrapping_add(u64::from(entity.interaction_count())),
            );
        }

        CommandResult::success(output)
    }
}