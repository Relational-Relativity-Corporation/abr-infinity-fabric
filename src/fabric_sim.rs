// fabric_sim.rs — Metatron Dynamics, Inc.
// Infinity Fabric traffic simulation under declared dependency structure.
// Bounded over D. No claim beyond D.
//
// ── Purpose ──────────────────────────────────────────────────────────────────
//
// This module declares three workload dependency classes and simulates their
// predicted Infinity Fabric traffic patterns. The simulation produces predicted
// DF PMC beat counts — specifically Coherent Station (CS) read/write beats —
// for comparison against hardware measurement via AMD uProf MSR mode.
//
// The simulation is declared as a model of declared relational structure.
// It does not claim to be hardware. The comparison between simulation output
// and hardware measurement is the experiment.
//
// ── Observable Grounding ─────────────────────────────────────────────────────
//
// DF PMC events used for hardware comparison are taken from:
//   Section 7.1 "Fabric Performance Monitor Counter (PMC) Events"
//   Processor Programming Reference (PPR) for AMD Family 19h Model 11h
//   Revision B1 (Zen 4 EPYC — same DF PMC architecture as Ryzen 7000 desktop)
//   Source: Linux kernel patch commit 5b2ca349c313 (Sandipan Das, AMD, 2022-12-14)
//   Observable provenance: AMD PPR through M.
//
// Target hardware observables (admissible on Ryzen 5 7600X via uProf MSR mode):
//
//   CS events (CCD↔IOD interface):
//     local_processor_read_data_beats_csN   — 64-byte read beats at CS N
//     local_processor_write_data_beats_csN  — 64-byte write beats at CS N
//     EventCode: 0x1f (CS0), 0x5f (CS1), ... Unit: DFPMC
//
//   CCM events (IOD internal — CPU Moderator):
//     local_socket_inf0_inbound_data_beats_ccmN  — 32-byte inbound beats at CCM N
//     local_socket_inf0_outbound_data_beats_ccmN — 64-byte outbound beats at CCM N
//     EventCode: 0x41e (CCM0 inf0 in), 0x41f (CCM0 inf1 in), ... Unit: DFPMC
//
//   xGMI link events (inter-socket — expected zero on single-socket desktop):
//     local_socket_outbound_data_beats_linkN — 64-byte outbound beats at xGMI link N
//     EventCode: 0xb5f (link0), 0xb9f (link1), ... Unit: DFPMC
//
// On Ryzen 5 7600X (single CCD, single socket):
//   - CS events measure the actual CCD↔IOD fabric edge declared in fabric_topology.rs
//   - CCM events measure IOD-internal routing
//   - xGMI events will return zero (no inter-socket links — confirms declared prediction)
//
// ── Scope Constraint (OC-IF-SIM-1) ───────────────────────────────────────────
//
// OC-IF-SIM-1 OPEN: Simulation results predict traffic at the IOD boundary
//   (CS events) on Ryzen 5 7600X. Correspondence to physical MI355X fabric
//   traffic (inter-OAM xGMI events) requires direct measurement on multi-die
//   hardware. The scope of this simulation-to-hardware comparison is:
//   declared relational structure → IOD memory controller bandwidth,
//   NOT inter-chiplet fabric traffic on MI355X.
//
// OC-IF-SIM-2 OPEN: Structural reduction (Phase 1) is declared and measurable
//   at any working-set scale. The interaction between structural reduction and
//   quadratic attention complexity is NOT modeled in Phase 1. At transformer
//   scale (n tokens, quadratic baseline = n² pairwise operations), IF traffic
//   scales as O(n²) for undeclared attention and O(n·k) for declared relational
//   attention where k is the admitted edge count. The absolute beat savings grow
//   quadratically with n. Phase 2 declares KV-cache traffic as the working set
//   and models the O(n²) → O(n·k) reduction at the fabric level.
//   Phase 1 establishes the structural reduction independently of complexity.
//
// ── Workload Class Declaration ────────────────────────────────────────────────
//
// Three dependency classes are declared. Each class defines a directed
// dependency graph over declared loci (data regions). The declared structure
// determines which data regions must be fetched across the CCD↔IOD boundary.
//
// Class A — Independent: zero inter-region declared dependencies.
//   Each locus's data is self-contained. No dependency edge crosses a
//   memory region boundary. Predicted CS beats: proportional to working
//   set size only (cold-start reads). No xGMI beats.
//
// Class B — Weakly Coupled: sparse inter-region declared dependencies.
//   Some loci declare dependencies on data in other memory regions.
//   Predicted CS beats: working set beats + inter-region dependency fetch beats.
//   Dependency fetch beats declared from edge count × data-per-edge.
//
// Class C — Fully Coupled: dense inter-region declared dependencies.
//   All loci declare dependencies on data across all other regions.
//   Predicted CS beats: working set beats + full cross-region dependency beats.
//   Maximum CS beat load for this workload scale.
//
// ── Measurement Pairing ───────────────────────────────────────────────────────
//
// Hardware measurement pairs:
//   CPI          — core execution cost (V13 grounding observable)
//   DRAM_PTI     — DRAM pressure (V13 grounding observable)
//   CS_READ_BEATS  — local_processor_read_data_beats_csN (sum across CS instances)
//   CS_WRITE_BEATS — local_processor_write_data_beats_csN (sum across CS instances)
//   CCM_IN_BEATS   — local_socket_inf0_inbound_data_beats_ccmN (sum)
//   XGMI_BEATS     — local_socket_outbound_data_beats_linkN (expected zero)
//
// Comparison structure:
//   Simulation predicts CS_READ_BEATS and XGMI_BEATS per workload class.
//   Hardware measures the same events via uProf MSR mode.
//   Correspondence: does declared dependency structure predict observed beat counts?
//
// ── Beat Unit Declaration ─────────────────────────────────────────────────────
//
// CS read beats: 64 bytes per beat (from PPR event description)
// CS write beats: 64 bytes per beat
// CCM inbound beats: 32 bytes per beat (inf0/inf1 interface)
// CCM outbound beats: 64 bytes per beat
// xGMI beats: 64 bytes per beat

use crate::fabric_topology::{declare_fabric_topology, FabricTopology};

/// Declared workload dependency class.
/// Each class defines the inter-region dependency structure of a workload.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorkloadClass {
    /// Class A — Independent: zero declared inter-region dependencies.
    /// Each data region is self-contained. No dependency edge crosses regions.
    Independent,

    /// Class B — Weakly Coupled: sparse declared inter-region dependencies.
    /// Each region declares dependencies on N_WEAK_DEPS other regions.
    WeaklyCoupled,

    /// Class C — Fully Coupled: dense declared inter-region dependencies.
    /// Each region declares dependencies on all other regions.
    FullyCoupled,
}

/// Declared workload parameters.
/// All values traceable to declared observables through M.
pub struct WorkloadParams {
    /// Number of declared data regions (analogous to OAM modules or CCDs).
    /// On Ryzen 5 7600X: 1 CCD with multiple L3 cache slices.
    /// Declared as 8 to match MI355X topology structure for comparison.
    pub n_regions: usize,

    /// Working set size per region in bytes.
    /// Declared from abr-datacenter-build: 1 MB per community analysis.
    /// For Ryzen comparison: varied across three working-set scales
    /// matching V13 Pass B (L2-resident, L3-resident, DRAM-resident).
    pub working_set_bytes_per_region: u64,

    /// Data transferred per declared dependency edge.
    /// Declared as one cache line (64 bytes) — the atomic unit of
    /// coherent data transfer across the fabric, per PPR beat definition.
    pub bytes_per_dependency_edge: u64,

    /// Number of declared dependency edges per region for WeaklyCoupled class.
    /// Declared as 2 — sparse coupling, each region depends on 2 others.
    pub n_weak_deps_per_region: usize,
}

impl WorkloadParams {
    /// Default declared parameters.
    /// Working set matches V13 Pass B 1× scale (L2-resident, 4 MB).
    pub fn default_l2() -> Self {
        WorkloadParams {
            n_regions: 8,
            working_set_bytes_per_region: 512_000, // 512 KB — 8 regions × 512 KB = 4 MB (L2)
            bytes_per_dependency_edge: 64,
            n_weak_deps_per_region: 2,
        }
    }

    /// L3-resident working set (V13 Pass B 2× scale — 32 MB).
    pub fn default_l3() -> Self {
        WorkloadParams {
            n_regions: 8,
            working_set_bytes_per_region: 4_000_000, // 4 MB — 8 × 4 MB = 32 MB (L3)
            bytes_per_dependency_edge: 64,
            n_weak_deps_per_region: 2,
        }
    }

    /// DRAM-resident working set (V13 Pass B 4× scale — 128 MB).
    pub fn default_dram() -> Self {
        WorkloadParams {
            n_regions: 8,
            working_set_bytes_per_region: 16_000_000, // 16 MB — 8 × 16 MB = 128 MB (DRAM)
            bytes_per_dependency_edge: 64,
            n_weak_deps_per_region: 2,
        }
    }
}

/// Declared dependency graph for a workload class.
/// Each edge represents a declared data dependency between regions.
/// Direction: dependent_region → source_region (data flows from source to dependent).
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    /// Region that declares the dependency (consumer).
    pub dependent: usize,
    /// Region whose data is required (provider).
    pub source: usize,
    /// Bytes of data transferred across this dependency edge.
    pub bytes: u64,
}

/// Build the declared dependency graph for a workload class.
/// All edges are declared from the workload class definition — not inferred.
pub fn declare_dependency_graph(
    class: WorkloadClass,
    params: &WorkloadParams,
) -> Vec<DependencyEdge> {
    let mut edges = Vec::new();

    match class {
        WorkloadClass::Independent => {
            // Zero inter-region dependencies declared.
            // No edges — each region's computation requires only its own data.
            // Declared prediction: CS read beats = working set reads only.
            // No cross-region data movement.
        }

        WorkloadClass::WeaklyCoupled => {
            // Each region i declares dependencies on the next N_WEAK_DEPS regions
            // (modular — wraps around). This models sparse relational coupling
            // where some observables depend on data from adjacent regions.
            for i in 0..params.n_regions {
                for k in 1..=params.n_weak_deps_per_region {
                    let source = (i + k) % params.n_regions;
                    if source != i {
                        edges.push(DependencyEdge {
                            dependent: i,
                            source,
                            bytes: params.bytes_per_dependency_edge,
                        });
                    }
                }
            }
        }

        WorkloadClass::FullyCoupled => {
            // Each region i declares dependencies on all other regions.
            // Models fully dense relational coupling — every observable
            // depends on data from every other region.
            for i in 0..params.n_regions {
                for j in 0..params.n_regions {
                    if i != j {
                        edges.push(DependencyEdge {
                            dependent: i,
                            source: j,
                            bytes: params.bytes_per_dependency_edge,
                        });
                    }
                }
            }
        }
    }

    edges
}

/// Predicted DF PMC beat counts from simulation.
/// All values are predictions from declared relational structure.
/// Units: beats (64 bytes per CS beat, per PPR declaration).
#[derive(Debug, Clone)]
pub struct PredictedBeatCounts {
    /// Workload class that produced this prediction.
    pub class: WorkloadClass,

    /// Working set scale label for identification.
    pub scale_label: &'static str,

    /// Working set bytes per region.
    pub working_set_bytes: u64,

    /// Total declared dependency edges in this workload.
    pub n_dependency_edges: usize,

    /// Predicted CS read beats from working set reads.
    /// Each region reads its working set from DRAM through the CCD↔IOD interface.
    /// Beats = ceil(working_set_bytes / 64) per region × n_regions.
    /// Declared from PPR: CS read beat = 64 bytes.
    pub predicted_cs_read_beats_working_set: u64,

    /// Predicted CS read beats from dependency edge data fetches.
    /// Each declared dependency edge requires a 64-byte fetch across the interface.
    /// Beats = n_dependency_edges × bytes_per_edge / 64.
    /// On Ryzen 5 7600X (single CCD): all dependency fetches route through L3/DRAM
    /// and appear as CS beats when data is not L3-resident.
    /// OC-IF-SIM-1: on MI355X, inter-region deps would generate xGMI beats.
    pub predicted_cs_read_beats_dependencies: u64,

    /// Total predicted CS read beats (working set + dependencies).
    pub predicted_cs_read_beats_total: u64,

    /// Predicted xGMI outbound beats.
    /// On Ryzen 5 7600X: declared as zero — no inter-socket links present.
    /// On MI355X: inter-region dependencies with regions on separate OAMs
    /// would generate xGMI beats. Declared prediction for desktop: 0.
    pub predicted_xgmi_beats: u64,

    /// Declared mechanism: does this workload class activate the
    /// lower-bandwidth fabric locus (the switch / IOD crossbar)?
    /// Independent: No — all data local.
    /// WeaklyCoupled: Partially — dependency fetches traverse IOD.
    /// FullyCoupled: Yes — full cross-region data movement through IOD.
    pub activates_lower_bandwidth_locus: bool,

    /// Undeclared baseline CS read beats.
    /// Declared as: a stateless system with no dependency information
    /// treats every region as coupled to every other — fetches all
    /// n_regions working sets for every region.
    /// Baseline = n_regions × ws_beats_per_region × n_regions
    ///          = n_regions² × ws_beats_per_region.
    /// This is the fabric traffic signature of quadratic attention
    /// (no declared structure — all tokens attend to all tokens).
    /// Phase 1 declares this as the structural baseline.
    /// OC-IF-SIM-2: interaction with O(n²) complexity declared for Phase 2.
    pub undeclared_baseline_beats: u64,

    /// Structural reduction factor: baseline / declared_total.
    /// Fixed by topology (n_regions) — independent of working set scale.
    /// At n_regions=8: factor=8.0 for Independent class.
    /// Interpretation: declared relational structure eliminates this
    /// fraction of CS read beats relative to the undeclared baseline.
    /// This is the hardware-measurable signature of ABR declared structure.
    pub structural_reduction_factor: f64,
}

/// Run simulation for one workload class at one working-set scale.
/// Returns predicted DF PMC beat counts.
pub fn simulate(
    class: WorkloadClass,
    params: &WorkloadParams,
    scale_label: &'static str,
    topology: &FabricTopology,
) -> PredictedBeatCounts {
    let dep_graph = declare_dependency_graph(class, params);
    let n_dependency_edges = dep_graph.len();

    // Predicted CS read beats from working set.
    // Each region reads its full working set; each 64-byte beat is one CS read.
    let ws_bytes_total = params.working_set_bytes_per_region * params.n_regions as u64;
    let cs_read_ws = (ws_bytes_total + 63) / 64; // ceil division

    // Predicted CS read beats from dependency edges.
    // Each declared dependency edge transfers bytes_per_dependency_edge bytes.
    let dep_bytes_total: u64 = dep_graph.iter().map(|e| e.bytes).sum();
    let cs_read_deps = (dep_bytes_total + 63) / 64;

    let cs_read_total = cs_read_ws + cs_read_deps;

    // Undeclared baseline: stateless system fetches all regions' data
    // for every region — equivalent to quadratic attention with no
    // declared dependency structure. Every region couples to every other.
    // Baseline CS read beats = n_regions² × ws_beats_per_region.
    let ws_beats_per_region = (params.working_set_bytes_per_region + 63) / 64;
    let undeclared_baseline_beats =
        (params.n_regions as u64) * (params.n_regions as u64) * ws_beats_per_region;

    // Structural reduction factor: how many times fewer beats does the
    // declared structure generate relative to the undeclared baseline?
    // Fixed by topology. Independent of working set scale.
    let structural_reduction_factor = if cs_read_total > 0 {
        undeclared_baseline_beats as f64 / cs_read_total as f64
    } else {
        1.0
    };

    // xGMI beats: declared zero on single-socket desktop.
    // On MI355X with inter-OAM dependencies, this would be non-zero.
    let xgmi_beats = 0u64;

    // Lower-bandwidth locus activation:
    // Independent: fabric switch not activated — no inter-region traffic.
    // WeaklyCoupled/FullyCoupled: dependency fetches traverse the IOD crossbar.
    let activates_lower_bandwidth_locus = match class {
        WorkloadClass::Independent => false,
        WorkloadClass::WeaklyCoupled => n_dependency_edges > 0,
        WorkloadClass::FullyCoupled => true,
    };

    // Verify topology is consistent with declared structure.
    // The declared fabric graph must pass ring inadmissibility.
    debug_assert!(
        topology.ring_inadmissibility_check(),
        "Fabric topology must pass ring inadmissibility before simulation"
    );

    PredictedBeatCounts {
        class,
        scale_label,
        working_set_bytes: params.working_set_bytes_per_region,
        n_dependency_edges,
        predicted_cs_read_beats_working_set: cs_read_ws,
        predicted_cs_read_beats_dependencies: cs_read_deps,
        predicted_cs_read_beats_total: cs_read_total,
        predicted_xgmi_beats: xgmi_beats,
        activates_lower_bandwidth_locus,
        undeclared_baseline_beats,
        structural_reduction_factor,
    }
}

/// Run the full simulation matrix:
/// 3 workload classes × 3 working-set scales = 9 simulation runs.
/// Returns all predicted beat counts for comparison against hardware.
pub fn run_simulation_matrix() -> Vec<PredictedBeatCounts> {
    let topology = declare_fabric_topology();
    let mut results = Vec::new();

    let scales: &[(&'static str, WorkloadParams)] = &[
        ("L2-resident (4MB)",   WorkloadParams::default_l2()),
        ("L3-resident (32MB)",  WorkloadParams::default_l3()),
        ("DRAM-resident (128MB)", WorkloadParams::default_dram()),
    ];

    let classes = [
        WorkloadClass::Independent,
        WorkloadClass::WeaklyCoupled,
        WorkloadClass::FullyCoupled,
    ];

    for (label, params) in scales {
        for &class in &classes {
            results.push(simulate(class, params, label, &topology));
        }
    }

    results
}

/// Format simulation results as a declared comparison table.
/// Columns map directly to DF PMC events for uProf measurement.
pub fn format_comparison_table(results: &[PredictedBeatCounts]) -> String {
    let mut out = String::new();
    out.push_str("Simulation → Hardware Comparison Table — Phase 1: Structural Reduction\n");
    out.push_str("Metatron Dynamics — abr-infinity-fabric\n");
    out.push_str("Predicted values from declared relational structure.\n");
    out.push_str("Hardware values: measure via uProf MSR mode, DF PMC events.\n");
    out.push_str("CS events: sum local_processor_read_data_beats_csN across all CS instances.\n");
    out.push_str("xGMI events: sum local_socket_outbound_data_beats_linkN (expected zero on desktop).\n");
    out.push_str("OC-IF-SIM-1 OPEN: scope limited to IOD boundary on Ryzen 7600X.\n");
    out.push_str("OC-IF-SIM-2 OPEN: O(n^2) complexity interaction declared for Phase 2.\n");
    out.push_str("Baseline: undeclared stateless system — all regions fetch all other regions.\n");
    out.push_str("Reduction factor: baseline_beats / declared_beats (fixed by topology).\n");
    out.push_str("\n");

    out.push_str(&format!(
        "{:<28} {:<16} {:>10} {:>14} {:>12} {:>14} {:>10}\n",
        "Scale", "Class", "Dep Edges",
        "Declared Beats", "Baseline", "Reduction", "xGMI"
    ));
    out.push_str(&"-".repeat(110));
    out.push('\n');

    for r in results {
        let class_str = match r.class {
            WorkloadClass::Independent   => "Independent",
            WorkloadClass::WeaklyCoupled => "WeaklyCoupled",
            WorkloadClass::FullyCoupled  => "FullyCoupled",
        };

        out.push_str(&format!(
            "{:<28} {:<16} {:>10} {:>14} {:>12} {:>13.1}x {:>10}\n",
            r.scale_label,
            class_str,
            r.n_dependency_edges,
            r.predicted_cs_read_beats_total,
            r.undeclared_baseline_beats,
            r.structural_reduction_factor,
            r.predicted_xgmi_beats,
        ));
    }

    out.push('\n');
    out.push_str("Hardware measurement columns to add:\n");
    out.push_str("  CS_READ_MEASURED   — sum of local_processor_read_data_beats_csN\n");
    out.push_str("  CS_WRITE_MEASURED  — sum of local_processor_write_data_beats_csN\n");
    out.push_str("  CCM_IN_MEASURED    — sum of local_socket_inf0_inbound_data_beats_ccmN\n");
    out.push_str("  XGMI_MEASURED      — sum of local_socket_outbound_data_beats_linkN\n");
    out.push_str("  CPI                — from V13 measurement protocol (core PMC)\n");
    out.push_str("  DRAM_PTI           — from V13 measurement protocol (core PMC)\n");

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_class_has_zero_dependency_edges() {
        let params = WorkloadParams::default_l2();
        let deps = declare_dependency_graph(WorkloadClass::Independent, &params);
        assert_eq!(deps.len(), 0,
            "Independent class must declare zero dependency edges");
    }

    #[test]
    fn independent_class_does_not_activate_fabric_locus() {
        let topology = declare_fabric_topology();
        let params = WorkloadParams::default_l2();
        let result = simulate(WorkloadClass::Independent, &params, "test", &topology);
        assert!(!result.activates_lower_bandwidth_locus,
            "Independent class must not activate lower-bandwidth fabric locus");
    }

    #[test]
    fn independent_class_xgmi_beats_are_zero() {
        let topology = declare_fabric_topology();
        let params = WorkloadParams::default_l2();
        let result = simulate(WorkloadClass::Independent, &params, "test", &topology);
        assert_eq!(result.predicted_xgmi_beats, 0,
            "Independent class must predict zero xGMI beats on single-socket hardware");
    }

    #[test]
    fn all_classes_xgmi_beats_are_zero_on_desktop() {
        // OC-IF-SIM-1: xGMI events are declared zero on single-socket desktop.
        let topology = declare_fabric_topology();
        for class in [
            WorkloadClass::Independent,
            WorkloadClass::WeaklyCoupled,
            WorkloadClass::FullyCoupled,
        ] {
            let result = simulate(class, &WorkloadParams::default_l2(), "test", &topology);
            assert_eq!(result.predicted_xgmi_beats, 0,
                "{:?}: all classes must predict zero xGMI beats on desktop (OC-IF-SIM-1)",
                class);
        }
    }

    #[test]
    fn weakly_coupled_has_correct_edge_count() {
        let params = WorkloadParams::default_l2();
        let deps = declare_dependency_graph(WorkloadClass::WeaklyCoupled, &params);
        // 8 regions × 2 weak deps each = 16 edges
        let expected = params.n_regions * params.n_weak_deps_per_region;
        assert_eq!(deps.len(), expected,
            "WeaklyCoupled must declare n_regions × n_weak_deps edges: \
             expected {}, got {}", expected, deps.len());
    }

    #[test]
    fn fully_coupled_has_correct_edge_count() {
        let params = WorkloadParams::default_l2();
        let deps = declare_dependency_graph(WorkloadClass::FullyCoupled, &params);
        // 8 regions × 7 others = 56 edges
        let expected = params.n_regions * (params.n_regions - 1);
        assert_eq!(deps.len(), expected,
            "FullyCoupled must declare n_regions × (n_regions-1) edges: \
             expected {}, got {}", expected, deps.len());
    }

    #[test]
    fn cs_read_beats_increase_with_dependency_count() {
        let topology = declare_fabric_topology();
        let params = WorkloadParams::default_dram();
        let ind = simulate(WorkloadClass::Independent,   &params, "test", &topology);
        let wk  = simulate(WorkloadClass::WeaklyCoupled, &params, "test", &topology);
        let fc  = simulate(WorkloadClass::FullyCoupled,  &params, "test", &topology);

        assert!(wk.predicted_cs_read_beats_total > ind.predicted_cs_read_beats_total,
            "WeaklyCoupled must predict more CS read beats than Independent");
        assert!(fc.predicted_cs_read_beats_total > wk.predicted_cs_read_beats_total,
            "FullyCoupled must predict more CS read beats than WeaklyCoupled");
    }

    #[test]
    fn cs_read_beats_increase_with_working_set_scale() {
        let topology = declare_fabric_topology();
        let l2   = simulate(WorkloadClass::Independent, &WorkloadParams::default_l2(),   "L2",   &topology);
        let l3   = simulate(WorkloadClass::Independent, &WorkloadParams::default_l3(),   "L3",   &topology);
        let dram = simulate(WorkloadClass::Independent, &WorkloadParams::default_dram(), "DRAM", &topology);

        assert!(l3.predicted_cs_read_beats_working_set > l2.predicted_cs_read_beats_working_set,
            "L3 scale must predict more CS read beats than L2 scale");
        assert!(dram.predicted_cs_read_beats_working_set > l3.predicted_cs_read_beats_working_set,
            "DRAM scale must predict more CS read beats than L3 scale");
    }

    #[test]
    fn fully_coupled_activates_lower_bandwidth_locus() {
        let topology = declare_fabric_topology();
        let result = simulate(WorkloadClass::FullyCoupled, &WorkloadParams::default_l2(), "test", &topology);
        assert!(result.activates_lower_bandwidth_locus,
            "FullyCoupled class must activate lower-bandwidth fabric locus");
    }

    #[test]
    fn independent_dependency_beats_are_zero() {
        let topology = declare_fabric_topology();
        let result = simulate(WorkloadClass::Independent, &WorkloadParams::default_l2(), "test", &topology);
        assert_eq!(result.predicted_cs_read_beats_dependencies, 0,
            "Independent class must predict zero dependency CS read beats");
    }

    #[test]
    fn no_self_loop_dependency_edges() {
        let params = WorkloadParams::default_l2();
        for class in [WorkloadClass::WeaklyCoupled, WorkloadClass::FullyCoupled] {
            let deps = declare_dependency_graph(class, &params);
            for edge in &deps {
                assert_ne!(edge.dependent, edge.source,
                    "{:?}: self-loop dependency edge at region {} — inadmissible",
                    class, edge.dependent);
            }
        }
    }

    #[test]
    fn simulation_matrix_produces_nine_results() {
        let results = run_simulation_matrix();
        assert_eq!(results.len(), 9,
            "Simulation matrix must produce 3 classes × 3 scales = 9 results");
    }

    #[test]
    fn topology_ring_inadmissibility_holds_through_simulation() {
        let topology = declare_fabric_topology();
        assert!(topology.ring_inadmissibility_check(),
            "Fabric topology must pass ring inadmissibility throughout simulation");
    }

    #[test]
    fn bytes_per_beat_consistent_with_ppr_declaration() {
        // PPR declares CS read beat = 64 bytes.
        // Working set of 640 bytes → ceil(640/64) = 10 beats.
        let params = WorkloadParams {
            n_regions: 1,
            working_set_bytes_per_region: 640,
            bytes_per_dependency_edge: 64,
            n_weak_deps_per_region: 0,
        };
        let topology = declare_fabric_topology();
        let result = simulate(WorkloadClass::Independent, &params, "test", &topology);
        assert_eq!(result.predicted_cs_read_beats_working_set, 10,
            "640 bytes / 64 bytes per beat must produce 10 CS read beats per PPR declaration");
    }

    #[test]
    fn undeclared_baseline_exceeds_declared_for_independent() {
        // Structural reduction: undeclared baseline must exceed declared total
        // for the Independent class at every working-set scale.
        let topology = declare_fabric_topology();
        for params in [
            WorkloadParams::default_l2(),
            WorkloadParams::default_l3(),
            WorkloadParams::default_dram(),
        ] {
            let result = simulate(WorkloadClass::Independent, &params, "test", &topology);
            assert!(result.undeclared_baseline_beats > result.predicted_cs_read_beats_total,
                "Undeclared baseline must exceed declared beats for Independent class");
        }
    }

    #[test]
    fn structural_reduction_factor_fixed_by_topology() {
        // Reduction factor is determined by n_regions, not working set size.
        // At n_regions=8, Independent class: baseline=8²×ws_beats, declared=8×ws_beats
        // → factor = 8.0 exactly.
        let topology = declare_fabric_topology();
        let r_l2   = simulate(WorkloadClass::Independent, &WorkloadParams::default_l2(),   "L2",   &topology);
        let r_l3   = simulate(WorkloadClass::Independent, &WorkloadParams::default_l3(),   "L3",   &topology);
        let r_dram = simulate(WorkloadClass::Independent, &WorkloadParams::default_dram(), "DRAM", &topology);

        let factor_l2   = (r_l2.structural_reduction_factor   * 10.0).round() / 10.0;
        let factor_l3   = (r_l3.structural_reduction_factor   * 10.0).round() / 10.0;
        let factor_dram = (r_dram.structural_reduction_factor * 10.0).round() / 10.0;

        assert_eq!(factor_l2, factor_l3,
            "Reduction factor must be identical across L2 and L3 scales");
        assert_eq!(factor_l3, factor_dram,
            "Reduction factor must be identical across L3 and DRAM scales");
        assert!(r_l2.structural_reduction_factor >= 7.9,
            "At n_regions=8, Independent reduction factor must be ~8.0, got {:.2}",
            r_l2.structural_reduction_factor);
    }

    #[test]
    fn reduction_factor_greater_than_one_for_all_classes() {
        // Every declared class must reduce traffic relative to undeclared baseline.
        let topology = declare_fabric_topology();
        for class in [
            WorkloadClass::Independent,
            WorkloadClass::WeaklyCoupled,
            WorkloadClass::FullyCoupled,
        ] {
            let result = simulate(class, &WorkloadParams::default_dram(), "test", &topology);
            assert!(result.structural_reduction_factor > 1.0,
                "{:?}: reduction factor must exceed 1.0, got {:.4}",
                class, result.structural_reduction_factor);
        }
    }

    #[test]
    fn independent_has_highest_reduction_factor() {
        // Independent class eliminates the most traffic — highest reduction factor.
        let topology = declare_fabric_topology();
        let ind = simulate(WorkloadClass::Independent,   &WorkloadParams::default_dram(), "test", &topology);
        let wk  = simulate(WorkloadClass::WeaklyCoupled, &WorkloadParams::default_dram(), "test", &topology);
        let fc  = simulate(WorkloadClass::FullyCoupled,  &WorkloadParams::default_dram(), "test", &topology);

        assert!(ind.structural_reduction_factor > wk.structural_reduction_factor,
            "Independent must have higher reduction factor than WeaklyCoupled");
        assert!(wk.structural_reduction_factor > fc.structural_reduction_factor,
            "WeaklyCoupled must have higher reduction factor than FullyCoupled");
    }
}
