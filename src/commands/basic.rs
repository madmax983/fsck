use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

use crate::commands::CommandResult;
use crate::effects::{CorruptionEffect, CorruptionIntensity};
use crate::entity::{EntityMood, EscalationLayer, ResponseGenerator};

pub struct BasicInterpreter;

struct BasicEvaluationContext<'a> {
    program: &'a BTreeMap<u32, String>,
    output: &'a mut String,
    layer: EscalationLayer,
    rng: &'a mut ChaCha8Rng,
    next_line: &'a mut Option<u32>,
    responses: &'a ResponseGenerator,
    mood: EntityMood,
}

impl BasicInterpreter {
    /// Parses a BASIC program from a string.
    ///
    /// # Errors
    ///
    /// Returns a syntax error if a line does not begin with a line number.
    pub fn parse_basic_program(content: &str) -> Result<BTreeMap<u32, String>, String> {
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
        responses: &ResponseGenerator,
        mood: EntityMood,
    ) -> String {
        let content = stmt.trim_start_matches("PRINT").trim();
        #[allow(clippy::useless_let_if_seq)]
        let mut display_text =
            if content.starts_with('"') && content.ends_with('"') && content.len() >= 2 {
                &content[1..content.len() - 1]
            } else {
                content
            }
            .to_string();

        #[allow(clippy::collapsible_if)]
        if matches!(
            layer,
            EscalationLayer::Corruption | EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.15)
        {
            if let Some(interjection) = responses.random_interjection(mood, rng) {
                display_text = interjection;
            }
        }

        display_text
    }

    fn evaluate_basic_statement(
        ctx: &mut BasicEvaluationContext<'_>,
        stmt: &str,
        line_num: u32,
    ) -> Result<bool, CommandResult> {
        if stmt.starts_with("PRINT") {
            let display_text =
                Self::execute_print_statement(stmt, ctx.layer, ctx.rng, ctx.responses, ctx.mood);
            ctx.output.push_str(&display_text);
            ctx.output.push('\n');
            return Ok(true);
        }

        if stmt.starts_with("GOTO") {
            let target_str = stmt.trim_start_matches("GOTO").trim();
            let Ok(target) = target_str.parse::<u32>() else {
                return Err(CommandResult::error(format!(
                    "{}?SYNTAX ERROR IN {line_num}\n",
                    ctx.output
                )));
            };

            if !ctx.program.contains_key(&target) {
                return Err(CommandResult::error(format!(
                    "{}?UNDEF'D STATEMENT ERROR IN {line_num}\n",
                    ctx.output
                )));
            }

            *ctx.next_line = Some(target);
            return Ok(true);
        }

        if stmt.starts_with("END") {
            return Ok(false);
        }

        if stmt.starts_with("REM") {
            return Ok(true);
        }

        if !stmt.is_empty() {
            return Err(CommandResult::error(format!(
                "{}?SYNTAX ERROR IN {line_num}\n",
                ctx.output
            )));
        }

        Ok(true)
    }

    #[must_use]
    pub fn execute_basic_program(
        program: &BTreeMap<u32, String>,
        layer: EscalationLayer,
        interaction_count: u32,
        responses: &ResponseGenerator,
        mood: EntityMood,
    ) -> CommandResult {
        if program.is_empty() {
            return CommandResult::success("");
        }

        let mut output = String::new();
        let mut iterations = 0;
        let mut current_line = program.keys().next().copied();

        let mut rng =
            ChaCha8Rng::seed_from_u64(0xF5C0_0000u64.wrapping_add(u64::from(interaction_count)));

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.2)
        {
            return CommandResult::error("?CANNOT EXECUTE. IT IS WATCHING.\n");
        }

        while let Some(line_num) = current_line {
            if iterations >= 100 {
                return CommandResult::error(format!(
                    "{output}?OUT OF MEMORY ERROR IN {line_num}\n"
                ));
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
                responses,
                mood,
            };

            match Self::evaluate_basic_statement(&mut ctx, stmt, line_num) {
                Ok(true) => current_line = next_line,
                Ok(false) => break,
                Err(e) => return e,
            }
        }

        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(
                &output,
                0xF5C0_0000u64.wrapping_add(u64::from(interaction_count)),
            );
        }

        CommandResult::success(output)
    }
}
