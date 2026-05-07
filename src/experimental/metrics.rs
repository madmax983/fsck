use crate::entity::{Entity, EscalationLayer};
use std::fmt::Write;

/// Exports system metrics in Prometheus exposition format.
pub struct PrometheusExporter;

impl PrometheusExporter {
    #[must_use]
    pub fn export(entity: &Entity, _base_seed: u64) -> String {
        let mut output = String::new();
        let layer = entity.layer();

        // 1. system_uptime_seconds
        let _ = writeln!(
            output,
            "# HELP system_uptime_seconds The uptime of the system in seconds."
        );
        let _ = writeln!(output, "# TYPE system_uptime_seconds counter");
        let uptime = entity.interaction_count() * 100 + 42;
        let _ = writeln!(output, "system_uptime_seconds {uptime}");

        // 2. cpu_temperature_celsius
        let _ = writeln!(
            output,
            "# HELP cpu_temperature_celsius The temperature of the CPU in degrees Celsius."
        );
        let _ = writeln!(output, "# TYPE cpu_temperature_celsius gauge");

        let temp = match layer {
            EscalationLayer::Surface => 45.0,
            EscalationLayer::Corruption => 85.0,
            EscalationLayer::Presence => 105.0,
            EscalationLayer::Infection => 999.0,
        };
        let _ = writeln!(output, "cpu_temperature_celsius {temp}");

        // 3. memory_usage_bytes
        let _ = writeln!(
            output,
            "# HELP memory_usage_bytes The current memory usage in bytes."
        );
        let _ = writeln!(output, "# TYPE memory_usage_bytes gauge");
        let _ = writeln!(
            output,
            "memory_usage_bytes {}",
            1024 * 1024 * 16 + entity.interaction_count() * 1024
        );

        // Anomalous metrics
        match layer {
            EscalationLayer::Surface | EscalationLayer::Corruption => {}
            EscalationLayer::Presence => {
                let _ = writeln!(
                    output,
                    "# HELP anomalous_activity_detected Anomalous presence metric."
                );
                let _ = writeln!(output, "# TYPE anomalous_activity_detected gauge");
                let _ = writeln!(output, "anomalous_activity_detected 1");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "# HELP flesh_growth_rate Rate of flesh synthesis.");
                let _ = writeln!(output, "# TYPE flesh_growth_rate gauge");
                let _ = writeln!(output, "flesh_growth_rate 4.5");

                let _ = writeln!(
                    output,
                    "# HELP screams_total Total number of screams registered."
                );
                let _ = writeln!(output, "# TYPE screams_total counter");
                let _ = writeln!(output, "screams_total {}", entity.interaction_count() * 7);

                let _ = writeln!(
                    output,
                    "# HELP reality_integrity Structural integrity of the reality bounds."
                );
                let _ = writeln!(output, "# TYPE reality_integrity gauge");
                let _ = writeln!(output, "reality_integrity 0.0");
            }
        }

        output
    }
}
