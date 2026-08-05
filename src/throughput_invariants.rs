// throughput_invariants.rs â€” Metatron Dynamics, Inc.
// Throughput derivation â€” closes OC-DB-3 structurally.
// Bounded over D. No claim beyond D.
//
// â”€â”€ What This Module Derives â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// With OC-DB-1 closed (partition mapping confirmed admissible), the
// throughput figure for OC-DB-3 is derivable from declared quantities.
//
// â”€â”€ Derivation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Declared quantities:
//   - Memory bandwidth per module: 8.0 TB/s = 8,000 GB/s (AMD spec through M)
//   - Community working set: 1 MB = 1,048,576 bytes (declared upper bound)
//   - ABR operators A â†’ B â†’ R: each reads the working set once (sparse traversal)
//   - Working set is resident in HBM3E: no eviction, full bandwidth available
//
// Derivation of operator throughput:
//   One ABR pass (A â†’ B â†’ R) reads the declared working set.
//   At declared HBM3E bandwidth of 8,000 GB/s:
//   Time per pass = working_set / bandwidth
//                 = 1,048,576 bytes / (8,000 Ã— 10â¹ bytes/s)
//                 = 1.311 Ã— 10â»â· seconds
//                 â‰ˆ 131.1 nanoseconds per community analysis per pass
//
//   Passes per second per module = 1 / time_per_pass
//                                = 8,000 Ã— 10â¹ / 1,048,576
//                                â‰ˆ 7,629,394 passes/second/module
//
//   Community analyses per second per module â‰ˆ 7.6 million.
//
// At full rack (8 modules):
//   Total throughput â‰ˆ 61 million community analyses per second.
//
// â”€â”€ Epistemic Status â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// This is a STRUCTURAL DERIVATION from declared constants â€” not a
// measured throughput. The derivation assumes:
//   1. Working set is fully resident in HBM3E (declared: fits in 288 GB).
//   2. Each ABR pass reads the working set once (sparse traversal property).
//   3. HBM3E bandwidth is fully available (no competing workloads).
//
// Assumption 2 is the open condition: the number of memory reads per ABR
// pass depends on the graph structure and is not yet formally derived
// from the operator mathematics. It is declared as "once" for sparse graphs
// where the working set fits in cache â€” but formal derivation is OC-DB-3.
//
// â”€â”€ Open Conditions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// OC-DB-3: Full closure requires instrument measurement. This derivation
//          closes OC-DB-3 structurally â€” the figure is admissible from
//          declared quantities. Correspondence requires measurement.
// OC-IF-1: Per-link bandwidth assumed uniform. Does not affect single-module
//          throughput for independent analyses (zero fabric traffic).
// OC-IF-3: HIP kernel implementation may introduce overhead not captured here.

use crate::partition_mapping::{PartitionResult, oc_db1_closed};
use crate::workload_graph::COMMUNITY_WORKING_SET_BYTES;
use crate::fabric_field::OAM_MEMORY_BW_GB_S;

/// Number of OAM modules in the declared platform.
const N_MODULES: u64 = 8;

/// Declared throughput invariants for the MI355X rack.
#[derive(Debug, Clone)]
pub struct ThroughputInvariants {
    /// Time per ABR pass per community analysis. Units: nanoseconds.
    /// Derived from working set / HBM3E bandwidth.
    pub time_per_pass_ns: f64,
    /// Community analyses per second per module.
    /// Derived from bandwidth / working set.
    pub analyses_per_second_per_module: f64,
    /// Total community analyses per second at rack scale (8 modules).
    pub analyses_per_second_rack: f64,
    /// Bandwidth utilization per module. Units: fraction of declared bandwidth.
    /// For resident working set: approaches 1.0 (full utilization).
    pub bandwidth_utilization_fraction: f64,
    /// Epistemic status of this derivation.
    pub epistemic_status: &'static str,
    /// Open conditions that apply.
    pub open_conditions: &'static str,
}

/// Derives throughput invariants from declared quantities.
///
/// Requires AdmissiblePass from partition_mapping â€” throughput derivation
/// is not admissible if OC-DB-1 is not closed.
///
/// Returns None if partition result does not close OC-DB-1.
pub fn derive_throughput(partition_result: &PartitionResult) -> Option<ThroughputInvariants> {
    if !oc_db1_closed(partition_result) {
        return None;
    }

    // Declared working set per community analysis â€” bytes.
    let working_set_bytes = COMMUNITY_WORKING_SET_BYTES as f64;

    // Declared HBM3E bandwidth per module â€” bytes/s.
    // OAM_MEMORY_BW_GB_S = 8,000 GB/s = 8,000 Ã— 10â¹ bytes/s
    let bandwidth_bytes_per_s = OAM_MEMORY_BW_GB_S * 1e9;

    // Time per ABR pass = working set / bandwidth.
    // Declaration: one ABR pass (A â†’ B â†’ R) reads the working set once.
    // This holds for sparse graphs where working set is cache-resident.
    let time_per_pass_s = working_set_bytes / bandwidth_bytes_per_s;
    let time_per_pass_ns = time_per_pass_s * 1e9;

    // Analyses per second per module = 1 / time_per_pass.
    let analyses_per_second_per_module = 1.0 / time_per_pass_s;

    // Total at rack scale.
    let analyses_per_second_rack = analyses_per_second_per_module * N_MODULES as f64;

    // Bandwidth utilization: for resident working set, full bandwidth is used.
    // Utilization = 1.0 when working set is fully resident and no eviction occurs.
    // Declared: working set (1 MB) << HBM3E (288 GB) â†’ resident â†’ utilization = 1.0.
    let bandwidth_utilization_fraction = 1.0;

    Some(ThroughputInvariants {
        time_per_pass_ns,
        analyses_per_second_per_module,
        analyses_per_second_rack,
        bandwidth_utilization_fraction,
        epistemic_status: "STRUCTURAL DERIVATION from declared constants. \
                            Not a measured throughput. Correspondence requires \
                            instrument measurement (OC-DB-3).",
        open_conditions: "OC-DB-3 (correspondence requires measurement), \
                          OC-IF-3 (HIP implementation overhead not captured)",
    })
}

/// Human-readable throughput report.
pub fn throughput_report(inv: &ThroughputInvariants) -> String {
    format!(
        "â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n\
         ABR INFINITY FABRIC â€” THROUGHPUT INVARIANTS\n\
         AMD Instinct MI355X Platform â€” Structural Derivation\n\
         Metatron Dynamics, Inc. Â· Bounded over D.\n\
         â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n\
         Time per ABR pass (Aâ†’Bâ†’R):           {:.1} ns per community\n\
         Throughput per module:               {:.0} analyses/second\n\
         Throughput at rack scale (8 modules):{:.0} analyses/second\n\
         Bandwidth utilization:               {:.0}% of declared HBM3E\n\
         â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
         Epistemic status: {}\n\
         â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
         Open conditions: {}\n\
         â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•",
        inv.time_per_pass_ns,
        inv.analyses_per_second_per_module,
        inv.analyses_per_second_rack,
        inv.bandwidth_utilization_fraction * 100.0,
        inv.epistemic_status,
        inv.open_conditions,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fabric_topology::declare_fabric_topology;
    use crate::workload_graph::declare_lompoc_community_graph;
    use crate::partition_mapping::{derive_partition_assignment, admissibility_check};

    fn get_admissible_result() -> PartitionResult {
        let g = declare_lompoc_community_graph();
        let t = declare_fabric_topology();
        let assignment = derive_partition_assignment(&g, &t);
        admissibility_check(&assignment, &g, &t)
    }

    #[test]
    fn throughput_requires_admissible_partition() {
        // derive_throughput must return None for a failed partition.
        let failed = PartitionResult::FailedCondition {
            condition: "AC-1",
            finding: "test".to_string(),
        };
        assert!(derive_throughput(&failed).is_none(),
            "Throughput derivation must require AdmissiblePass input");
    }

    #[test]
    fn throughput_derived_from_admissible_partition() {
        let result = get_admissible_result();
        let inv = derive_throughput(&result);
        assert!(inv.is_some(),
            "Throughput must be derivable from admissible partition result");
    }

    #[test]
    fn time_per_pass_derived_correctly() {
        // time = 1,048,576 bytes / (8,000 Ã— 10â¹ bytes/s) = 131.1 ns
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        let expected_ns = COMMUNITY_WORKING_SET_BYTES as f64 / (OAM_MEMORY_BW_GB_S * 1e9) * 1e9;
        assert!((inv.time_per_pass_ns - expected_ns).abs() < 1e-3,
            "Time per pass must be derived from working set / bandwidth: \
             got {:.1} ns, expected {:.1} ns", inv.time_per_pass_ns, expected_ns);
    }

    #[test]
    fn throughput_per_module_large() {
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        // ~7.6 million analyses/second per module
        assert!(inv.analyses_per_second_per_module > 1_000_000.0,
            "Throughput per module must exceed 1M analyses/second");
    }

    #[test]
    fn rack_throughput_is_eight_modules() {
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        let expected = inv.analyses_per_second_per_module * 8.0;
        assert!((inv.analyses_per_second_rack - expected).abs() < 1.0,
            "Rack throughput must equal 8 Ã— per-module throughput");
    }

    #[test]
    fn bandwidth_utilization_declared_correctly() {
        // For resident working set, bandwidth utilization is declared as 1.0.
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        assert!((inv.bandwidth_utilization_fraction - 1.0).abs() < 1e-10,
            "Bandwidth utilization must be 1.0 for resident working set");
    }

    #[test]
    fn open_conditions_named() {
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        assert!(inv.open_conditions.contains("OC-DB-3"),
            "OC-DB-3 must be named in open conditions");
    }

    #[test]
    fn report_states_structural_derivation() {
        let result = get_admissible_result();
        let inv = derive_throughput(&result).unwrap();
        let report = throughput_report(&inv);
        assert!(report.contains("STRUCTURAL DERIVATION"),
            "Report must state epistemic status as structural derivation");
        assert!(report.contains("OC-DB-3"),
            "Report must name OC-DB-3");
    }
}
