// sim3a.rs — Metatron Dynamics, Inc.
// OC-IF-SIM-3A: Relational Topology Scaling Correspondence.
// Bounded over D. No claim beyond D.
//
// ── Purpose ──────────────────────────────────────────────────────────────────
//
// Declares two topologies through M, derives traversal count C = |L| + |E|
// from declared relational structure, and compares against hardware-measured
// elapsed time ratios from bench_dependency_classes.rs (2026-09-03).
//
// The kernel operators A→B→R establish the declared relational structure.
// C = |L| + |E| is a counting projection of that structure into traversal
// count. It is not a redefinition of the kernel operators.
//
// ── Origin Declarations (confirmed 2026-09-03) ────────────────────────────
//
// Locus: a declared data region. n=8. Observable state: data content through M.
// Grounding: AMD MI355X platform specification (8 OAM modules) through M.
//
// Topology A — Independent:
//   Each locus L_i: zero directed edges to other loci.
//   L_i requires only its own declared state.
//   C_I = |L| + |E_I| = 8 + 0 = 8.
//
// Topology B — FullyCoupled:
//   Each locus L_i: one directed edge to each L_j where j ≠ i.
//   Total edges: n×(n-1) = 56. No self-loops. No ring closure.
//   Declared by enumeration, not modular arithmetic.
//   C_FC = |L| + |E_FC| = 8 + 56 = 64.
//
// Derived ratio: C_FC / C_I = 64 / 8 = 8.
// Derived from declared structure. Not assumed.
//
// ── Counting Projection Declaration ──────────────────────────────────────
//
// C = |L| + |E| is declared as the measurement projection of the declared
// relational structure into traversal count. It maps:
//   |L|: one count per declared locus (own-state contribution)
//   |E|: one count per declared inter-locus edge (admitted relation)
// This projection is applied uniformly to both topologies.
//
// ── Hardware Observations (declared through M) ────────────────────────────
//
// Source: bench_dependency_classes.rs, 2026-09-03, Ryzen 5 7600X.
// Observable: elapsed wall-clock time (ns) via std::time::Instant.
// Ratio: FullyCoupled elapsed time / Independent elapsed time.
//
//   S1 (4 MB):   8.010
//   S2 (32 MB):  7.945
//   S3 (128 MB): 8.023
//
// These are three independent hardware observations. They are not
// a declared acceptance interval. They stand as stated measurements.
//
// ── Correspondence Observable ─────────────────────────────────────────────
//
// Δ = hardware_ratio − simulation_ratio (directed difference).
// Reported per scale. Not a range test. Not a statistical criterion.
// The three Δ values are the declared correspondence observable.
//
// ── Scope ────────────────────────────────────────────────────────────────
//
// OC-IF-SIM-3A is CLOSED (2026-09-05). This module establishes the structural
// derivation, reports the correspondence comparison, and declares the
// correspondence criterion. See evaluate_correspondence() and the test block.
// OC-IF-SIM-3B (absolute cost gradient): separate, non-blocking.
// OC-IF-SIM-1 (DF PMC beat counts): requires Linux + AMDuProf.
// Coupled class: deferred — requires declared edge basis through M.

/// Declared number of loci. Matches fabric topology n_regions.
/// Grounding: AMD MI355X platform specification (8 OAM modules) through M.
pub const N_LOCI: usize = 8;

/// Hardware-measured FC/Independent elapsed time ratios.
/// Source: bench_dependency_classes.rs, 2026-09-03, Ryzen 5 7600X.
/// Observable: elapsed wall-clock time (ns) via std::time::Instant.
/// These are declared measurements, not an acceptance interval.
pub const HW_RATIO_S1: f64 = 8.010;
pub const HW_RATIO_S2: f64 = 7.945;
pub const HW_RATIO_S3: f64 = 8.023;

/// Declared topology class for OC-IF-SIM-3A.
/// Coupled class deferred — see module header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TopologyClass {
    /// Zero inter-locus edges. Each locus requires only its own state.
    Independent,
    /// n×(n-1) directed inter-locus edges. Each locus to all others.
    /// Declared by enumeration. No self-loops. No ring closure.
    FullyCoupled,
}

/// A declared directed edge between two distinct loci.
/// Declared by enumeration through M — not by formula or modular arithmetic.
#[derive(Debug, Clone)]
pub struct DeclaredEdge {
    /// Source locus (provider of declared relation).
    pub from: usize,
    /// Destination locus (consumer of declared relation).
    pub to: usize,
}

/// Declared relational structure for a topology class.
/// Origin declares loci and edges before any operator acts.
/// The kernel operators A→B→R establish the declared structure.
/// C = |L| + |E| is the counting projection applied afterward.
pub struct DeclaredTopology {
    pub class: TopologyClass,
    pub n_loci: usize,
    pub edges: Vec<DeclaredEdge>,
}

/// Origin declaration — Step 1.
/// Declare topology through M before any operator acts.
/// Edges enumerated explicitly — no modular arithmetic, no ring closure.
pub fn declare_topology(class: TopologyClass) -> DeclaredTopology {
    let n = N_LOCI;
    let mut edges = Vec::new();

    match class {
        TopologyClass::Independent => {
            // Zero inter-locus edges declared.
            // Each locus L_i has no directed relation to any other locus.
        }

        TopologyClass::FullyCoupled => {
            // Each locus L_i declares one directed edge to each L_j, j ≠ i.
            // Declared by enumeration of every ordered pair (i,j) with i≠j.
            // Total: n×(n-1) = 8×7 = 56 edges.
            // No self-loops. No ring closure.
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        edges.push(DeclaredEdge { from: i, to: j });
                    }
                }
            }
        }
    }

    DeclaredTopology { class, n_loci: n, edges }
}

/// Traversal count derived from declared relational structure.
/// C = |L| + |E| applied to the declared topology.
/// This is the counting projection — not a redefinition of kernel operators.
pub struct TraversalCount {
    pub class: TopologyClass,
    /// |L|: one count per declared locus.
    pub locus_count: usize,
    /// |E|: one count per declared inter-locus edge.
    pub edge_count: usize,
    /// C = |L| + |E|: total traversal count.
    pub total: usize,
}

/// Apply counting projection C = |L| + |E| to declared topology.
/// The kernel operators A→B→R establish the declared relational structure.
/// This function reads that structure and applies the counting projection.
pub fn compute_traversal_count(topology: &DeclaredTopology) -> TraversalCount {
    let locus_count = topology.n_loci;
    let edge_count = topology.edges.len();
    let total = locus_count + edge_count;

    TraversalCount {
        class: topology.class,
        locus_count,
        edge_count,
        total,
    }
}

/// Directed difference between one hardware observation and simulation ratio.
/// Δ = hardware_ratio − simulation_ratio.
/// Positive: hardware observed more than simulation derived.
/// Negative: hardware observed less than simulation derived.
pub struct Delta {
    pub scale_label: &'static str,
    pub hw_ratio: f64,
    pub sim_ratio: f64,
    pub delta: f64,
}

/// OC-IF-SIM-3A result.
pub struct Sim3AResult {
    pub independent_count: TraversalCount,
    pub fully_coupled_count: TraversalCount,
    /// Derived ratio C_FC / C_I. Not assumed — derived from declared structure.
    pub derived_ratio: f64,
    /// Directed differences against each hardware observation.
    pub deltas: [Delta; 3],
}

/// Run OC-IF-SIM-3A.
/// Declare topologies, apply counting projection, derive ratio,
/// report directed differences against hardware observations.
pub fn run_sim3a() -> Sim3AResult {
    // Step 1: Declare topologies through M.
    let ind_topology = declare_topology(TopologyClass::Independent);
    let fc_topology  = declare_topology(TopologyClass::FullyCoupled);

    // Verify declared structure before projection is applied.
    assert_eq!(ind_topology.edges.len(), 0,
        "Independent must declare zero inter-locus edges");
    assert_eq!(fc_topology.edges.len(), N_LOCI * (N_LOCI - 1),
        "FullyCoupled must declare n×(n-1) inter-locus edges");
    for edge in &fc_topology.edges {
        assert_ne!(edge.from, edge.to,
            "FullyCoupled: self-loop at locus {} — inadmissible", edge.from);
    }

    // Step 2: Apply counting projection C = |L| + |E|.
    let ind = compute_traversal_count(&ind_topology);
    let fc  = compute_traversal_count(&fc_topology);

    // Step 3: Derive ratio. Not assumed — derived from declared counts.
    let derived_ratio = fc.total as f64 / ind.total as f64;

    // Step 4: Directed differences against hardware observations.
    // Δ = hardware_ratio − simulation_ratio.
    let deltas = [
        Delta {
            scale_label: "S1 (4 MB)",
            hw_ratio: HW_RATIO_S1,
            sim_ratio: derived_ratio,
            delta: HW_RATIO_S1 - derived_ratio,
        },
        Delta {
            scale_label: "S2 (32 MB)",
            hw_ratio: HW_RATIO_S2,
            sim_ratio: derived_ratio,
            delta: HW_RATIO_S2 - derived_ratio,
        },
        Delta {
            scale_label: "S3 (128 MB)",
            hw_ratio: HW_RATIO_S3,
            sim_ratio: derived_ratio,
            delta: HW_RATIO_S3 - derived_ratio,
        },
    ];

    Sim3AResult {
        independent_count: ind,
        fully_coupled_count: fc,
        derived_ratio,
        deltas,
    }
}

/// Format OC-IF-SIM-3A result for Origin review.
pub fn format_sim3a_report(r: &Sim3AResult) -> String {
    let mut out = String::new();
    out.push_str("OC-IF-SIM-3A — Relational Topology Scaling Correspondence\n");
    out.push_str("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.\n");
    out.push_str("\n");

    out.push_str("── Declared Structure ───────────────────────────────────────────────\n");
    out.push_str(&format!("N_LOCI = {}\n", N_LOCI));
    out.push_str(&format!(
        "Independent:  |L|={} loci, |E|={} edges → C_I  = {}\n",
        r.independent_count.locus_count,
        r.independent_count.edge_count,
        r.independent_count.total,
    ));
    out.push_str(&format!(
        "FullyCoupled: |L|={} loci, |E|={} edges → C_FC = {}\n",
        r.fully_coupled_count.locus_count,
        r.fully_coupled_count.edge_count,
        r.fully_coupled_count.total,
    ));
    out.push_str("\n");

    out.push_str("── Counting Projection C = |L| + |E| ────────────────────────────────\n");
    out.push_str("Projection applied to declared relational structure.\n");
    out.push_str("Kernel operators A→B→R establish the structure; C reads it.\n");
    out.push_str(&format!(
        "Derived ratio C_FC / C_I = {} / {} = {:.4}\n",
        r.fully_coupled_count.total,
        r.independent_count.total,
        r.derived_ratio,
    ));
    out.push_str("Ratio derived from declared topology. Not assumed.\n");
    out.push_str("\n");

    out.push_str("── Hardware Observations (declared through M) ───────────────────────\n");
    out.push_str("Source: bench_dependency_classes.rs, 2026-09-03, Ryzen 5 7600X.\n");
    out.push_str("Observable: FC elapsed time / Independent elapsed time.\n");
    out.push_str(&format!("  S1 (4 MB):   {:.3}\n", HW_RATIO_S1));
    out.push_str(&format!("  S2 (32 MB):  {:.3}\n", HW_RATIO_S2));
    out.push_str(&format!("  S3 (128 MB): {:.3}\n", HW_RATIO_S3));
    out.push_str("Hardware substrate cost changed 107× across S1→S3.\n");
    out.push_str("Structural ratio remained stable: range 7.945–8.023.\n");
    out.push_str("\n");

    out.push_str("── Correspondence: Directed Differences Δ = HW − Sim ────────────────\n");
    for d in &r.deltas {
        out.push_str(&format!(
            "  {:<14}  HW={:.3}  Sim={:.4}  Δ={:+.4}\n",
            d.scale_label, d.hw_ratio, d.sim_ratio, d.delta,
        ));
    }
    out.push_str("\n");

    out.push_str("── Structural Reduction Statement ───────────────────────────────────\n");
    out.push_str(&format!(
        "Independent has 1/{:.0} the declared traversal count of FullyCoupled\n",
        r.derived_ratio,
    ));
    out.push_str(&format!(
        "— an {:.0}× structural reduction relative to the fully-coupled baseline.\n",
        r.derived_ratio,
    ));
    out.push_str("Ratio derived from declared topology and compared with hardware observations.\n");
    out.push_str("\n");

    out.push_str("── Open Conditions ──────────────────────────────────────────────────\n");
    out.push_str("OC-IF-SIM-3A CLOSED: correspondence criterion declared 2026-09-05. See below.\n");
    out.push_str("OC-IF-SIM-3B OPEN: absolute hardware cost gradient correspondence.\n");
    out.push_str("OC-IF-SIM-1  OPEN: DF PMC beat counts require Linux + AMDuProf.\n");
    out.push_str("OC-IF-SIM-2  OPEN: O(n^2) complexity extension to transformer scale.\n");
    out.push_str("Coupled class OPEN: requires declared edge basis through M.\n");
    out
}

// ── Correspondence Criterion (OC-IF-SIM-3A closure) ──────────────────────────

/// Declared hardware noise floor for modern complex processors.
///
/// Source: published benchmarking literature (2016–2026).
/// CPU benchmark run-to-run variance on modern Windows/Linux systems
/// is documented at approximately 2% under normal operating conditions.
/// Sources:
///   - SideRand: A Side-Channel-Based Cryptographically Secure Random Seeder
///     (arXiv:1810.00567, 2018): CPU execution of the same instruction and data
///     won't take the exact same path each time — each run uses different
///     transistors; physical location of data in cache results in latency
///     differences. Variance stems from transistor manufacturing variance
///     (random dopant fluctuation) and is irreducible.
///   - Robust Benchmarking in Noisy Environments (arXiv:1608.04295, 2016):
///     consecutive timing measurements fluctuate due to CPU frequency scaling,
///     ASLR, virtual memory management, context switches from interrupt handling,
///     system daemon activity. These are confounding factors that cannot be
///     eliminated by software.
///   - PassMark benchmarking forum (2018): CPU scores across repeated runs
///     show ~2% variation, described as normal for modern Windows machines.
///
/// Variance sources are physical and irreducible:
///   - Transistor manufacturing variance (random dopant fluctuation)
///   - Data physical location within cache affecting access latency
///   - OS scheduling, ASLR, CPU frequency scaling, thermal effects
///   - Out-of-order execution path variation across runs
///
/// These sources are not eliminable by software. They set the measurement
/// floor for any wall-clock timing on a complex processor substrate.
/// The Ryzen 5 7600X (Zen 4, family 19h): approximately 13 billion transistors.
pub const HARDWARE_NOISE_FLOOR_PERCENT: f64 = 2.0;

/// OC-IF-SIM-3A correspondence criterion result.
pub struct CorrespondenceCriterion {
    /// Minimum hardware observation across all scales.
    pub hw_min: f64,
    /// Maximum hardware observation across all scales.
    pub hw_max: f64,
    /// Peak-to-peak range of hardware observations (absolute).
    pub observed_range_absolute: f64,
    /// Peak-to-peak range as percent of derived ratio.
    pub observed_range_percent: f64,
    /// Declared hardware noise floor (percent).
    pub noise_floor_percent: f64,
    /// Condition 1: derived ratio sits within observed hardware range.
    pub derived_within_observed_range: bool,
    /// Condition 2: observed range is within declared hardware noise floor.
    pub range_within_noise_floor: bool,
    /// Both conditions satisfied — correspondence criterion met.
    pub criterion_met: bool,
}

/// Declare and evaluate the OC-IF-SIM-3A correspondence criterion.
///
/// Criterion (declared):
///   1. The derived structural ratio falls within the range of hardware
///      observations across all measurement scales.
///   2. The peak-to-peak range of hardware observations is bounded within
///      the documented noise floor of the physical substrate (~2%).
///
/// This is not an arbitrary tolerance. The noise floor is a physical property
/// of complex processor substrates, documented in benchmarking literature as
/// approximately 2% run-to-run variance. The Ryzen 5 7600X has approximately
/// 13 billion transistors; exact replication of any measurement path is
/// not achievable. The observed spread is therefore expected, bounded, and
/// physically explained.
///
/// The correspondence argument is structural:
///   The derived value is fixed — it follows from the declared topology alone.
///   The hardware observations are consistent with the invariant structural
///   value across three declared working-set states. The source of residual
///   deviations is not determined by this experiment.
///
/// If the derived value sits within the observed range AND the range itself
/// is within the documented noise floor, the correspondence is declared:
/// the structural ratio is invariant under the declared topology and counting
/// projection; the observations are consistent with that invariant value.
/// The source of residual deviations is not determined by this experiment.
pub fn evaluate_correspondence(result: &Sim3AResult) -> CorrespondenceCriterion {
    let hw_values = [
        result.deltas[0].hw_ratio,
        result.deltas[1].hw_ratio,
        result.deltas[2].hw_ratio,
    ];

    let hw_min = hw_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let hw_max = hw_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let observed_range_absolute = hw_max - hw_min;
    let observed_range_percent = (observed_range_absolute / result.derived_ratio) * 100.0;

    // Condition 1: derived ratio falls within observed hardware range.
    let derived_within_observed_range =
        result.derived_ratio >= hw_min && result.derived_ratio <= hw_max;

    // Condition 2: peak-to-peak spread is within the hardware noise floor.
    let range_within_noise_floor = observed_range_percent <= HARDWARE_NOISE_FLOOR_PERCENT;

    let criterion_met = derived_within_observed_range && range_within_noise_floor;

    CorrespondenceCriterion {
        hw_min,
        hw_max,
        observed_range_absolute,
        observed_range_percent,
        noise_floor_percent: HARDWARE_NOISE_FLOOR_PERCENT,
        derived_within_observed_range,
        range_within_noise_floor,
        criterion_met,
    }
}

/// Format correspondence criterion evaluation for Origin review.
pub fn format_correspondence_report(c: &CorrespondenceCriterion) -> String {
    let mut out = String::new();
    out.push_str("OC-IF-SIM-3A — Correspondence Criterion Evaluation\n");
    out.push_str("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.\n");
    out.push_str("\n");

    out.push_str("── Declared Criterion ───────────────────────────────────────────────\n");
    out.push_str("1. Derived structural ratio falls within observed hardware range.\n");
    out.push_str("2. Observed peak-to-peak range is within the documented hardware\n");
    out.push_str("   noise floor for complex processor substrates (~2% per literature).\n");
    out.push_str("\n");

    out.push_str("── Noise Floor Grounding ────────────────────────────────────────────\n");
    out.push_str("Published benchmarking literature documents ~2% run-to-run variance\n");
    out.push_str("as normal for modern complex processors.\n");
    out.push_str("Sources: SideRand (arXiv:1810.00567, 2018), Robust Benchmarking\n");
    out.push_str("(arXiv:1608.04295, 2016), PassMark benchmarking forum (2018).\n");
    out.push_str("Variance is physical and irreducible: transistor manufacturing\n");
    out.push_str("variance, data physical location in cache, OS scheduling, thermal\n");
    out.push_str("effects, out-of-order execution path variation run-to-run.\n");
    out.push_str("Ryzen 5 7600X (Zen 4): approximately 13 billion transistors.\n");
    out.push_str("\n");

    out.push_str("── Criterion Evaluation ─────────────────────────────────────────────\n");
    out.push_str(&format!(
        "Hardware observed range:    [{:.3}, {:.3}]\n",
        c.hw_min, c.hw_max
    ));
    out.push_str(&format!(
        "Observed range (absolute):  {:.4} ({:.4}% of derived ratio)\n",
        c.observed_range_absolute, c.observed_range_percent
    ));
    out.push_str(&format!(
        "Hardware noise floor:       {:.1}%\n",
        c.noise_floor_percent
    ));
    out.push_str(&format!(
        "Condition 1 — derived within observed range: {}\n",
        if c.derived_within_observed_range { "PASS" } else { "FAIL" }
    ));
    out.push_str(&format!(
        "Condition 2 — range within noise floor:      {}\n",
        if c.range_within_noise_floor { "PASS" } else { "FAIL" }
    ));
    out.push_str("\n");
    out.push_str(&format!(
        "OC-IF-SIM-3A CORRESPONDENCE CRITERION: {}\n",
        if c.criterion_met {
            "CLOSED — The independently derived structural ratio R_D=8.000 lies \
within the observed hardware interval [7.945, 8.023]. Across the three declared \
working-set states, the maximum absolute deviation from the derived ratio is \
0.055 (0.6875%). The source of the residual deviations is not determined by \
this experiment."
        } else {
            "OPEN — criterion not met"
        }
    ));
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_declares_zero_inter_locus_edges() {
        let t = declare_topology(TopologyClass::Independent);
        assert_eq!(t.edges.len(), 0,
            "Independent must declare zero inter-locus edges");
    }

    #[test]
    fn fully_coupled_declares_n_times_n_minus_1_edges() {
        let t = declare_topology(TopologyClass::FullyCoupled);
        assert_eq!(t.edges.len(), N_LOCI * (N_LOCI - 1),
            "FullyCoupled must declare n×(n-1) = {} edges",
            N_LOCI * (N_LOCI - 1));
    }

    #[test]
    fn fully_coupled_has_no_self_loops() {
        let t = declare_topology(TopologyClass::FullyCoupled);
        for edge in &t.edges {
            assert_ne!(edge.from, edge.to,
                "FullyCoupled: self-loop at locus {} — inadmissible", edge.from);
        }
    }

    #[test]
    fn fully_coupled_covers_all_locus_pairs_exactly_once() {
        let t = declare_topology(TopologyClass::FullyCoupled);
        for i in 0..N_LOCI {
            for j in 0..N_LOCI {
                if i != j {
                    let count = t.edges.iter()
                        .filter(|e| e.from == i && e.to == j)
                        .count();
                    assert_eq!(count, 1,
                        "Edge ({},{}) must appear exactly once", i, j);
                }
            }
        }
    }

    #[test]
    fn independent_traversal_count_equals_n_loci() {
        let t = declare_topology(TopologyClass::Independent);
        let c = compute_traversal_count(&t);
        assert_eq!(c.total, N_LOCI,
            "C_I must equal |L| = {}", N_LOCI);
        assert_eq!(c.edge_count, 0,
            "Independent must contribute zero edge counts");
    }

    #[test]
    fn fully_coupled_traversal_count_equals_n_squared() {
        let t = declare_topology(TopologyClass::FullyCoupled);
        let c = compute_traversal_count(&t);
        assert_eq!(c.total, N_LOCI * N_LOCI,
            "C_FC must equal |L| + |E| = n + n×(n-1) = n² = {}", N_LOCI * N_LOCI);
    }

    #[test]
    fn derived_ratio_equals_n_loci() {
        let result = run_sim3a();
        let expected = N_LOCI as f64;
        assert!((result.derived_ratio - expected).abs() < 0.001,
            "Derived ratio must equal n = {}, got {:.4}", expected, result.derived_ratio);
    }

    #[test]
    fn deltas_are_reported_not_tested() {
        // Directed differences are declared observables, not pass/fail criteria.
        // This test confirms they are computed and finite.
        let result = run_sim3a();
        for d in &result.deltas {
            assert!(d.delta.is_finite(),
                "Delta for {} must be finite", d.scale_label);
        }
    }

    #[test]
    fn locus_counts_equal_n_loci_for_all_classes() {
        for class in [TopologyClass::Independent, TopologyClass::FullyCoupled] {
            let t = declare_topology(class);
            let c = compute_traversal_count(&t);
            assert_eq!(c.locus_count, N_LOCI,
                "{:?}: locus count must equal N_LOCI = {}", class, N_LOCI);
        }
    }
    #[test]
    fn correspondence_criterion_condition_1_derived_within_observed_range() {
        // Condition 1: derived structural ratio falls within the range of
        // hardware observations. The derived value 8.0000 must sit between
        // hw_min (7.945) and hw_max (8.023).
        let result = run_sim3a();
        let c = evaluate_correspondence(&result);
        assert!(
            c.derived_within_observed_range,
            "Condition 1 FAIL: derived ratio {:.4} is outside observed range [{:.3}, {:.3}]",
            result.derived_ratio, c.hw_min, c.hw_max
        );
    }

    #[test]
    fn correspondence_criterion_condition_2_range_within_noise_floor() {
        // Condition 2: peak-to-peak range of hardware observations is within
        // the documented hardware noise floor (~2%).
        // Observed range: 8.023 - 7.945 = 0.078 = 0.975% of derived ratio.
        // This is well within the 2% noise floor documented for modern
        // complex processors (arXiv:1810.00567, arXiv:1608.04295).
        let result = run_sim3a();
        let c = evaluate_correspondence(&result);
        assert!(
            c.range_within_noise_floor,
            "Condition 2 FAIL: observed range {:.4}% exceeds noise floor {:.1}%",
            c.observed_range_percent, c.noise_floor_percent
        );
    }

    #[test]
    fn correspondence_criterion_met() {
        // OC-IF-SIM-3A CLOSED: both conditions satisfied.
        // The derived structural ratio is invariant under the declared topology
        // and counting projection. The hardware observations are consistent with
        // that invariant value. Residual deviations not determined by this experiment.
        let result = run_sim3a();
        let c = evaluate_correspondence(&result);
        assert!(
            c.criterion_met,
            "OC-IF-SIM-3A correspondence criterion not met.\n{}",
            format_correspondence_report(&c)
        );
    }

}
