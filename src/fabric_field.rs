// fabric_field.rs â€” Metatron Dynamics, Inc.
// Bandwidth field over Infinity Fabric topology â€” A operator application.
// Bounded over D. No claim beyond D.
//
// â”€â”€ Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// The fabric field declares available bandwidth at each locus through M.
// Source: AMD MI355X Platform specification.
//
// Locus bandwidth values:
//   OAM module: memory bandwidth per module (8.0 TB/s = 8,000 GB/s).
//     Source: AMD MI355X Platform specification â€” "Memory Bandwidth: 8.0 TB/s Per OAM"
//     Observable provenance: AMD published platform specification through M.
//
//   Fabric switch: aggregate bi-directional P2P bandwidth (1,194.8 GB/s).
//     Source: AMD MI355X Platform specification.
//     Observable provenance: AMD published platform specification through M.
//
// â”€â”€ A Operator â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// A(x)[e] = x[source(e)] - x[target(e)]
// Consistent with kernel declaration in operators.rs V8.
//
// At OAMâ†’Switch edges:
//   OAM bandwidth (8,000 GB/s) - switch aggregate (1,194.8 GB/s) = +6,805.2 GB/s
//   Positive A: the module source has higher bandwidth than the switch target.
//   This is the declared fabric bottleneck â€” the switch is the constraint,
//   not the modules. The positive A at these edges identifies where
//   relational contrast is highest in the fabric.
//
// At Switchâ†’OAM edges:
//   Switch bandwidth (1,194.8 GB/s) - OAM bandwidth (8,000 GB/s) = -6,805.2 GB/s
//   Negative A: the switch source has lower bandwidth than the module target.
//   The receiving module can absorb more than the switch delivers.
//
// The antisymmetry (equal magnitude, opposite sign) is the declared
// structural property of the crossbar topology â€” both directions of the
// same physical constraint, expressed as distinct relations (D-2).

use crate::fabric_topology::{FabricTopology, FABRIC_SWITCH, OAM_BASE,
                               FABRIC_AGGREGATE_BW_GB_S};

/// Memory bandwidth per OAM module.
/// Source: AMD MI355X Platform specification.
///   "Memory Bandwidth: 8.0 TB/s Per OAM"
///   https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html
/// Observable provenance: AMD published platform specification through M.
/// Units: GB/s (converted from 8.0 TB/s Ã— 1000 GB/TB).
pub const OAM_MEMORY_BW_GB_S: f64 = 8_000.0; // 8.0 TB/s

/// Declared bandwidth field over fabric loci.
#[derive(Debug, Clone)]
pub struct FabricBandwidthField {
    /// Available bandwidth at each declared locus. Units: GB/s.
    /// Index corresponds to locus index in FabricTopology.
    pub bandwidth_gb_s: Vec<f64>,
}

/// Declares the bandwidth field at declared operating parameters.
/// All values trace to AMD MI355X Platform specification through M.
pub fn declare_bandwidth_field(n_loci: usize) -> FabricBandwidthField {
    let mut bandwidth_gb_s = vec![0.0; n_loci];

    // Each OAM module: declared memory bandwidth.
    // Observable provenance: AMD MI355X Platform specification through M.
    for i in 0..8 {
        bandwidth_gb_s[OAM_BASE + i] = OAM_MEMORY_BW_GB_S;
    }

    // Fabric switch: declared aggregate P2P bandwidth.
    // Observable provenance: AMD MI355X Platform specification through M.
    bandwidth_gb_s[FABRIC_SWITCH] = FABRIC_AGGREGATE_BW_GB_S;

    FabricBandwidthField { bandwidth_gb_s }
}

/// A operator over fabric edges: directed bandwidth difference.
///
/// A(x)[e] = x[source(e)] - x[target(e)]
///
/// Consistent with kernel declaration in operators.rs V8.
/// Positive output: source has higher bandwidth than target.
/// At OAMâ†’Switch: module bandwidth >> switch bandwidth â†’ A > 0.
/// This identifies the switch as the bandwidth bottleneck (declared constraint).
pub fn operator_a_fabric(
    field: &FabricBandwidthField,
    topology: &FabricTopology,
) -> Vec<f64> {
    topology.edges.iter().map(|e| {
        field.bandwidth_gb_s[e.source] - field.bandwidth_gb_s[e.target]
    }).collect()
}

/// Fabric contrast: identifies which edges carry the highest relational contrast.
/// High positive A at OAMâ†’Switch edges identifies the switch as the constraint.
/// Returns (edge_index, source, target, a_value) sorted by |A| descending.
pub fn fabric_contrast(
    a_output: &[f64],
    topology: &FabricTopology,
) -> Vec<(usize, usize, usize, f64)> {
    let mut contrasts: Vec<(usize, usize, usize, f64)> = a_output.iter()
        .enumerate()
        .map(|(i, &a)| (i, topology.edges[i].source, topology.edges[i].target, a))
        .collect();
    contrasts.sort_by(|a, b| b.3.abs().partial_cmp(&a.3.abs()).unwrap());
    contrasts
}

/// Bottleneck identification: returns the locus that is the binding
/// bandwidth constraint â€” the locus where A output is most consistently
/// positive (more bandwidth leaving than arriving).
///
/// In the declared MI355X topology, this is FABRIC_SWITCH: all 8 modules
/// can generate 8,000 GB/s each, but the switch handles only 1,194.8 GB/s
/// aggregate. The switch is the declared bottleneck.
pub fn identify_bottleneck(
    field: &FabricBandwidthField,
    topology: &FabricTopology,
) -> usize {
    // The bottleneck locus is the one where total egress demand from upstream
    // exceeds its declared bandwidth. Find the locus with minimum bandwidth
    // that has incoming edges from high-bandwidth sources.
    let a = operator_a_fabric(field, topology);

    // Locus where sum of incoming A values is most positive
    // (most bandwidth pressure from upstream)
    let mut pressure = vec![0.0f64; topology.n_loci];
    for (idx, edge) in topology.edges.iter().enumerate() {
        // Positive A at edge e = source has more bandwidth than target.
        // Pressure accumulates at the target (the constrained locus).
        if a[idx] > 0.0 {
            pressure[edge.target] += a[idx];
        }
    }
    pressure.iter().enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(FABRIC_SWITCH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fabric_topology::declare_fabric_topology;

    #[test]
    fn oam_modules_at_declared_bandwidth() {
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        for i in 0..8 {
            assert!((f.bandwidth_gb_s[OAM_BASE + i] - OAM_MEMORY_BW_GB_S).abs() < 1e-6,
                "OAM module {} must be at declared memory bandwidth", i);
        }
    }

    #[test]
    fn switch_at_declared_aggregate_bandwidth() {
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        assert!((f.bandwidth_gb_s[FABRIC_SWITCH] - FABRIC_AGGREGATE_BW_GB_S).abs() < 1e-6,
            "Switch must be at declared aggregate P2P bandwidth");
    }

    #[test]
    fn a_operator_sign_consistent_with_kernel() {
        // Kernel declares A(x)[e] = x[source] - x[target].
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        let a = operator_a_fabric(&f, &t);
        for (idx, edge) in t.edges.iter().enumerate() {
            let expected = f.bandwidth_gb_s[edge.source] - f.bandwidth_gb_s[edge.target];
            assert!((a[idx] - expected).abs() < 1e-6,
                "A operator must equal x[source] - x[target] at edge {}", idx);
        }
    }

    #[test]
    fn a_positive_at_module_to_switch_edges() {
        // OAM (8,000 GB/s) â†’ Switch (1,194.8 GB/s): A = 8000 - 1194.8 = +6805.2
        // Positive A identifies switch as bandwidth bottleneck.
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        let a = operator_a_fabric(&f, &t);
        // First 8 edges are OAMâ†’Switch
        for i in 0..8 {
            assert!(a[i] > 0.0,
                "A at OAM_{}â†’Switch must be positive: \
                 module bandwidth > switch bandwidth", i);
        }
    }

    #[test]
    fn a_negative_at_switch_to_module_edges() {
        // Switch (1,194.8 GB/s) â†’ OAM (8,000 GB/s): A = 1194.8 - 8000 = -6805.2
        // Negative A: module can absorb more than switch delivers.
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        let a = operator_a_fabric(&f, &t);
        // Edges 8..15 are Switchâ†’OAM
        for i in 8..16 {
            assert!(a[i] < 0.0,
                "A at Switchâ†’OAM must be negative: \
                 switch bandwidth < module bandwidth");
        }
    }

    #[test]
    fn a_antisymmetric_across_directions() {
        // |A at OAMâ†’Switch| == |A at Switchâ†’OAM| for same module pair.
        // This is the declared structural antisymmetry of the crossbar.
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        let a = operator_a_fabric(&f, &t);
        for i in 0..8 {
            let module_to_switch = a[i];
            let switch_to_module = a[i + 8];
            assert!((module_to_switch + switch_to_module).abs() < 1e-6,
                "A values must be equal and opposite for module {} \
                 bidirectional pair", i);
        }
    }

    #[test]
    fn bottleneck_identified_as_switch() {
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        let bottleneck = identify_bottleneck(&f, &t);
        assert_eq!(bottleneck, FABRIC_SWITCH,
            "Declared bottleneck must be FABRIC_SWITCH â€” \
             switch aggregate bandwidth is the binding constraint");
    }

    #[test]
    fn bandwidth_field_finite() {
        let t = declare_fabric_topology();
        let f = declare_bandwidth_field(t.n_loci);
        assert!(f.bandwidth_gb_s.iter().all(|v| v.is_finite()),
            "All bandwidth field values must be finite");
    }

    #[test]
    fn oam_bandwidth_declared_from_specification() {
        // 8.0 TB/s = 8,000 GB/s â€” direct from AMD MI355X platform spec.
        assert!((OAM_MEMORY_BW_GB_S - 8_000.0).abs() < 1e-6,
            "OAM memory bandwidth must be 8,000 GB/s (8.0 TB/s from AMD spec)");
    }
}
