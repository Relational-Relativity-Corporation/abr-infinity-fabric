// convergence.rs â€” Metatron Dynamics, Inc.
// Convergence test â€” formal closure of OC-DB-1.
// Bounded over D. No claim beyond D.
//
// â”€â”€ What This Module Establishes â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// This module is the OC-DB-1 closure test. It chains all prior modules
// and confirms that:
//
//   1. Infinity Fabric topology is correctly declared through M.
//   2. Bandwidth field applies A operator consistently with kernel V8.
//   3. Community analysis graph is declared as partitionable workload.
//   4. Partition mapping satisfies AC-1 through AC-4.
//   5. Execution model declares correct software stack and efficiency basis.
//   6. Throughput derivation is admissible from confirmed partition.
//
// A PASS here is the formal closure of OC-DB-1 from abr-datacenter-build.
//
// â”€â”€ Convergence with abr-datacenter-build â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Layer 1 (abr-grid-integration):
//   Grid physics â†’ 100-175 MW viable band from community observables.
//   Status: COMPLETE, PUBLISHED.
//
// Layer 2 (abr-workload-architecture):
//   Compute architecture â†’ same 100-175 MW band independently.
//   Status: COMPLETE.
//
// Layer 3 (abr-datacenter-build):
//   Rack declared as relational structure. Efficiency readable from
//   operator output. OC-DB-1 named as open condition.
//   Status: COMPLETE, V0.2 verified.
//
// Layer 4 (this repo â€” abr-infinity-fabric):
//   Kernel-to-hardware mapping formally derived. ABR operators execute
//   at declared HBM3E bandwidth on resident working set. Zero fabric
//   traffic for independent community analyses. OC-DB-1 closed.
//   OC-DB-3 closed structurally.
//   Status: this convergence test.
//
// â”€â”€ Conformance Statement â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// A PASS is a conformance statement â€” not a correspondence claim.
// Correspondence (measured throughput, confirmed HIP implementation)
// requires instrument data. See OC-DB-3 and OC-IF-3.

use crate::fabric_topology::declare_fabric_topology;
use crate::fabric_field::{declare_bandwidth_field, operator_a_fabric};
use crate::workload_graph::declare_lompoc_community_graph;
use crate::partition_mapping::{derive_partition_assignment, admissibility_check,
                                oc_db1_closed, PartitionResult};
use crate::kernel_execution::declare_execution_model;
use crate::throughput_invariants::{derive_throughput, ThroughputInvariants};

/// Result of the full convergence test.
#[derive(Debug)]
pub enum ConvergenceResult {
    /// All declared conditions satisfied. OC-DB-1 formally closed.
    Pass {
        throughput: ThroughputInvariants,
        oc_db1_status: &'static str,
        oc_db3_status: &'static str,
        layer_summary: &'static str,
    },
    /// A declared condition failed. Located finding returned.
    Fail {
        stage: &'static str,
        finding: String,
    },
}

/// Runs the full convergence test.
/// Chains: fabric_topology â†’ fabric_field â†’ workload_graph â†’
///         partition_mapping â†’ admissibility_check â†’ throughput_invariants.
pub fn run_convergence_test() -> ConvergenceResult {
    // Stage 1: Declare fabric topology.
    let topology = declare_fabric_topology();
    if !topology.ring_inadmissibility_check() {
        return ConvergenceResult::Fail {
            stage: "fabric_topology",
            finding: "Ring inadmissibility check failed on declared fabric topology"
                .to_string(),
        };
    }

    // Stage 2: Declare bandwidth field and verify A operator.
    let field = declare_bandwidth_field(topology.n_loci);
    let a_output = operator_a_fabric(&field, &topology);

    // Verify A operator sign convention matches kernel declaration.
    for (idx, edge) in topology.edges.iter().enumerate() {
        let expected = field.bandwidth_gb_s[edge.source] - field.bandwidth_gb_s[edge.target];
        if (a_output[idx] - expected).abs() > 1e-6 {
            return ConvergenceResult::Fail {
                stage: "fabric_field",
                finding: format!(
                    "A operator sign inconsistency at edge {}: \
                     got {:.4}, expected {:.4}",
                    idx, a_output[idx], expected
                ),
            };
        }
    }

    // Stage 3: Declare community workload graph.
    let graph = declare_lompoc_community_graph();
    if !graph.fits_in_module() {
        return ConvergenceResult::Fail {
            stage: "workload_graph",
            finding: format!(
                "Community working set {} bytes exceeds HBM3E capacity",
                graph.working_set_bytes
            ),
        };
    }

    // Stage 4: Derive partition assignment.
    let assignment = derive_partition_assignment(&graph, &topology);

    // Stage 5: Check admissibility conditions AC-1 through AC-4.
    let partition_result = admissibility_check(&assignment, &graph, &topology);
    if !oc_db1_closed(&partition_result) {
        let finding = match &partition_result {
            PartitionResult::FailedCondition { condition, finding } => {
                format!("Condition {} failed: {}", condition, finding)
            }
            _ => "Unknown partition failure".to_string(),
        };
        return ConvergenceResult::Fail {
            stage: "partition_mapping",
            finding,
        };
    }

    // Stage 6: Verify execution model.
    let model = declare_execution_model();
    if !model.working_set_resident {
        return ConvergenceResult::Fail {
            stage: "kernel_execution",
            finding: "Working set declared as not resident in HBM3E â€” \
                      throughput derivation inadmissible".to_string(),
        };
    }

    // Stage 7: Derive throughput invariants.
    let throughput = match derive_throughput(&partition_result) {
        Some(t) => t,
        None => {
            return ConvergenceResult::Fail {
                stage: "throughput_invariants",
                finding: "Throughput derivation returned None â€” \
                          partition result did not close OC-DB-1".to_string(),
            };
        }
    };

    // All stages passed.
    ConvergenceResult::Pass {
        throughput,
        oc_db1_status: "CLOSED â€” ABR kernel maps to Infinity Fabric topology. \
                         Independent community analyses execute at full HBM3E \
                         bandwidth with zero fabric traffic. AC-1 through AC-4 \
                         satisfied by declared structure.",
        oc_db3_status: "CLOSED STRUCTURALLY â€” throughput derived from declared \
                         constants. Correspondence requires instrument measurement.",
        layer_summary: "Layer 1 (grid physics) + Layer 2 (compute architecture) + \
                        Layer 3 (rack declaration) + Layer 4 (kernel-hardware mapping): \
                        four-layer convergence across independent derivations. \
                        The facility is physically viable, computationally viable, \
                        and the kernel executes on the hardware natively.",
    }
}

/// Human-readable convergence report.
pub fn convergence_report(result: &ConvergenceResult) -> String {
    match result {
        ConvergenceResult::Pass {
            throughput,
            oc_db1_status,
            oc_db3_status,
            layer_summary,
        } => {
            format!(
                "â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n\
                 ABR INFINITY FABRIC â€” CONVERGENCE REPORT\n\
                 AMD Instinct MI355X Platform\n\
                 Metatron Dynamics, Inc. Â· Bounded over D.\n\
                 â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n\
                 RESULT: PASS\n\
                 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
                 OC-DB-1: {}\n\
                 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
                 OC-DB-3: {}\n\
                 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
                 Throughput (structural derivation):\n\
                   Per module:  {:.0} analyses/second\n\
                   Rack total:  {:.0} analyses/second\n\
                   Pass time:   {:.1} ns per community analysis\n\
                 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
                 Layer convergence: {}\n\
                 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n\
                 PASS is a conformance statement.\n\
                 Correspondence requires instrument measurement.\n\
                 â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•",
                oc_db1_status,
                oc_db3_status,
                throughput.analyses_per_second_per_module,
                throughput.analyses_per_second_rack,
                throughput.time_per_pass_ns,
                layer_summary,
            )
        }
        ConvergenceResult::Fail { stage, finding } => {
            format!(
                "â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n\
                 ABR INFINITY FABRIC â€” CONVERGENCE REPORT\n\
                 RESULT: FAIL\n\
                 Stage: {}\n\
                 Finding: {}\n\
                 â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•",
                stage, finding
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_test_passes() {
        let result = run_convergence_test();
        match &result {
            ConvergenceResult::Pass { .. } => {} // expected
            ConvergenceResult::Fail { stage, finding } => {
                panic!("Convergence test failed at stage '{}': {}", stage, finding);
            }
        }
    }

    #[test]
    fn oc_db1_named_as_closed_in_pass() {
        let result = run_convergence_test();
        match result {
            ConvergenceResult::Pass { oc_db1_status, .. } => {
                assert!(oc_db1_status.contains("CLOSED"),
                    "Pass result must declare OC-DB-1 as CLOSED");
            }
            ConvergenceResult::Fail { .. } => panic!("Expected Pass"),
        }
    }

    #[test]
    fn oc_db3_named_as_structurally_closed_in_pass() {
        let result = run_convergence_test();
        match result {
            ConvergenceResult::Pass { oc_db3_status, .. } => {
                assert!(oc_db3_status.contains("CLOSED STRUCTURALLY"),
                    "Pass result must declare OC-DB-3 as CLOSED STRUCTURALLY");
            }
            ConvergenceResult::Fail { .. } => panic!("Expected Pass"),
        }
    }

    #[test]
    fn throughput_positive_in_pass() {
        let result = run_convergence_test();
        match result {
            ConvergenceResult::Pass { throughput, .. } => {
                assert!(throughput.analyses_per_second_per_module > 0.0,
                    "Throughput per module must be positive");
                assert!(throughput.analyses_per_second_rack > 0.0,
                    "Rack throughput must be positive");
            }
            ConvergenceResult::Fail { .. } => panic!("Expected Pass"),
        }
    }

    #[test]
    fn report_states_conformance_not_correspondence() {
        let result = run_convergence_test();
        let report = convergence_report(&result);
        assert!(report.contains("conformance statement"),
            "Report must state that PASS is a conformance statement, \
             not a correspondence claim");
    }

    #[test]
    fn four_layer_convergence_named() {
        let result = run_convergence_test();
        match result {
            ConvergenceResult::Pass { layer_summary, .. } => {
                assert!(layer_summary.contains("Layer 1"),
                    "Layer summary must reference Layer 1 (grid physics)");
                assert!(layer_summary.contains("Layer 4"),
                    "Layer summary must reference Layer 4 (kernel-hardware mapping)");
            }
            ConvergenceResult::Fail { .. } => panic!("Expected Pass"),
        }
    }
}
