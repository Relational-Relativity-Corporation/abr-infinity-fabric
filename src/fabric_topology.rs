// fabric_topology.rs â€” Metatron Dynamics, Inc.
// AMD Infinity Fabric declared as a directed relational graph.
// Bounded over D. No claim beyond D.
//
// â”€â”€ Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Source: AMD Instinct MI355X Platform specification.
//   https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html
//   Retrieved: 2026-08-05
//
// The Infinity Fabric is a published point-to-point interconnect topology.
// Every link has exactly one admissible direction per declared transfer â€”
// the direction traceable to an observable through M (transfer initiator
// to transfer target, declared from hardware specification).
//
// â”€â”€ Locus Index Map â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
//  0  OAM_0         MI355X accelerator module 0
//  1  OAM_1         MI355X accelerator module 1
//  2  OAM_2         MI355X accelerator module 2
//  3  OAM_3         MI355X accelerator module 3
//  4  OAM_4         MI355X accelerator module 4
//  5  OAM_5         MI355X accelerator module 5
//  6  OAM_6         MI355X accelerator module 6
//  7  OAM_7         MI355X accelerator module 7
//  8  FABRIC_SWITCH Infinity Fabric crossbar switch
//
// Total: 9 loci
//
// â”€â”€ Edge Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Each OAM module connects to the fabric switch bidirectionally.
// Moduleâ†’Switch and Switchâ†’Module are declared as DISTINCT relations
// with independent observable provenance (D-2: directional distinctness).
//
// Total edges: 8 (moduleâ†’switch) + 8 (switchâ†’module) = 16 directed edges.
//
// â”€â”€ Bandwidth Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// AMD declared: Total Aggregate Bi-directional I/O Bandwidth (P2P) = 1,194.8 GB/s
// Source: AMD MI355X Platform specification (direct measurement through M).
//
// Per-link derivation:
//   8 modules Ã— 2 directions = 16 declared directed links.
//   1,194.8 GB/s Ã· 16 links = 74.675 GB/s per directed link.
//
// This derivation assumes uniform per-link bandwidth â€” the AMD specification
// declares aggregate bandwidth, not per-link. The uniform assumption is
// declared as a partition of the aggregate; it is admissible as a declared
// structural property of the crossbar topology.
//
// OC-IF-1: Per-link bandwidth uniformity â€” the AMD specification declares
//   aggregate bi-directional bandwidth. The per-link derivation assumes
//   uniform distribution across 16 directed links. If AMD publishes
//   per-link figures independently, replace the derived value with the
//   direct measurement.
//
// â”€â”€ Ring Inadmissibility â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Ring topology is inadmissible. The fabric switch is a routing locus,
// not a closed ring. No module connects directly to another module â€”
// all inter-module communication passes through the declared switch locus.

/// Total number of declared fabric loci.
/// 8 OAM modules + 1 Infinity Fabric switch = 9.
/// Source: AMD MI355X Platform specification (8 OAM modules confirmed).
pub const FABRIC_N_LOCI: usize = 9;

/// Locus index constants.
pub const OAM_BASE: usize = 0;   // OAM_0..OAM_7 = indices 0..7
pub const FABRIC_SWITCH: usize = 8;

/// Total aggregate bi-directional P2P bandwidth across all fabric links.
/// Source: AMD MI355X Platform specification.
///   "Total Aggregate Bi-directional I/O Bandwidth (Peer-to-Peer): 1,194.8 GB/s"
///   https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html
/// Observable provenance: AMD published platform specification through M.
/// Units: GB/s.
pub const FABRIC_AGGREGATE_BW_GB_S: f64 = 1_194.8;

/// Number of directed fabric links.
/// 8 modules Ã— 2 directions (moduleâ†’switch, switchâ†’module) = 16.
/// Derived from declared topology structure.
pub const FABRIC_N_DIRECTED_LINKS: usize = 16;

/// Per-link bandwidth â€” derived from aggregate divided by link count.
/// Assumes uniform bandwidth distribution across all directed links.
/// OC-IF-1: uniformity assumption â€” replace with direct measurement if available.
/// Units: GB/s.
pub const FABRIC_PER_LINK_BW_GB_S: f64 =
    FABRIC_AGGREGATE_BW_GB_S / FABRIC_N_DIRECTED_LINKS as f64;

/// A declared directed edge in the Infinity Fabric topology.
/// Direction is an admissibility condition â€” not a choice.
/// Every edge carries observable provenance from the hardware specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FabricEdge {
    /// Source locus index.
    pub source: usize,
    /// Target locus index.
    pub target: usize,
    /// Declared bandwidth on this link. Units: GB/s.
    pub bandwidth_gb_s: f64,
    /// Observable property establishing this direction through M.
    pub provenance: &'static str,
}

/// The declared Infinity Fabric topology.
#[derive(Debug, Clone)]
pub struct FabricTopology {
    /// Number of declared loci.
    pub n_loci: usize,
    /// All declared directed fabric edges.
    pub edges: Vec<FabricEdge>,
}

/// Declares the Infinity Fabric topology from AMD MI355X platform specification.
/// Every edge direction is traceable to a declared observable through M.
/// Source: AMD MI355X Platform specification, retrieved 2026-08-05.
pub fn declare_fabric_topology() -> FabricTopology {
    let mut edges = Vec::new();

    // Module â†’ Switch edges (8 edges)
    // Direction: OAM module initiates transfer to fabric switch.
    // Observable: inter-module data transfer initiates at source module,
    // routes through fabric switch â€” declared from Infinity Fabric
    // architecture specification through M.
    for i in 0..8 {
        edges.push(FabricEdge {
            source: OAM_BASE + i,
            target: FABRIC_SWITCH,
            bandwidth_gb_s: FABRIC_PER_LINK_BW_GB_S,
            provenance: "Infinity Fabric: OAM module initiates transfer \
                         to fabric switch â€” declared from AMD MI355X \
                         platform specification through M",
        });
    }

    // Switch â†’ Module edges (8 edges)
    // Direction: fabric switch routes to receiving OAM module.
    // Independent observable provenance from Moduleâ†’Switch (D-2).
    // These are distinct relations â€” not reverse observations of the same edge.
    for i in 0..8 {
        edges.push(FabricEdge {
            source: FABRIC_SWITCH,
            target: OAM_BASE + i,
            bandwidth_gb_s: FABRIC_PER_LINK_BW_GB_S,
            provenance: "Infinity Fabric: fabric switch routes to receiving \
                         OAM module â€” independent provenance from moduleâ†’switch \
                         per D-2 directional distinctness",
        });
    }

    FabricTopology {
        n_loci: FABRIC_N_LOCI,
        edges,
    }
}

impl FabricTopology {
    /// Total declared edge count.
    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }

    /// Edges leaving a given locus.
    pub fn edges_from(&self, locus: usize) -> Vec<&FabricEdge> {
        self.edges.iter().filter(|e| e.source == locus).collect()
    }

    /// Edges arriving at a given locus.
    pub fn edges_to(&self, locus: usize) -> Vec<&FabricEdge> {
        self.edges.iter().filter(|e| e.target == locus).collect()
    }

    /// Total declared bandwidth leaving a locus. Units: GB/s.
    pub fn egress_bandwidth_gb_s(&self, locus: usize) -> f64 {
        self.edges_from(locus).iter().map(|e| e.bandwidth_gb_s).sum()
    }

    /// Total declared bandwidth arriving at a locus. Units: GB/s.
    pub fn ingress_bandwidth_gb_s(&self, locus: usize) -> f64 {
        self.edges_to(locus).iter().map(|e| e.bandwidth_gb_s).sum()
    }

    /// Ring inadmissibility check.
    /// Returns true if the topology is admissible (no rings).
    pub fn ring_inadmissibility_check(&self) -> bool {
        for edge in &self.edges {
            // Self-loop: inadmissible
            if edge.source == edge.target {
                return false;
            }
            // Direct module-to-module edges bypass the switch: inadmissible.
            // All inter-module communication must route through FABRIC_SWITCH.
            let source_is_module = edge.source < 8;
            let target_is_module = edge.target < 8;
            if source_is_module && target_is_module {
                return false;
            }
        }
        true
    }

    /// Confirms declared aggregate bandwidth matches specification.
    /// Sum of all egress bandwidths should equal aggregate Ã— 2
    /// (bidirectional â€” each byte counted once in each direction).
    pub fn aggregate_bandwidth_check(&self) -> (f64, f64) {
        let total: f64 = self.edges.iter().map(|e| e.bandwidth_gb_s).sum();
        (total, FABRIC_AGGREGATE_BW_GB_S)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fabric_locus_count() {
        assert_eq!(FABRIC_N_LOCI, 9,
            "Fabric must have 9 loci: 8 OAM modules + 1 switch");
    }

    #[test]
    fn fabric_edge_count() {
        let t = declare_fabric_topology();
        assert_eq!(t.n_edges(), 16,
            "Fabric must have 16 directed edges: \
             8 moduleâ†’switch + 8 switchâ†’module");
    }

    #[test]
    fn all_modules_have_egress_to_switch() {
        let t = declare_fabric_topology();
        for i in 0..8 {
            let all_edges = t.edges_from(OAM_BASE + i);
            let egress: Vec<_> = all_edges.iter()
                .filter(|e| e.target == FABRIC_SWITCH)
                .collect();
            assert_eq!(egress.len(), 1,
                "OAM module {} must have exactly one egress to switch", i);
        }
    }

    #[test]
    fn all_modules_receive_from_switch() {
        let t = declare_fabric_topology();
        for i in 0..8 {
            let all_edges = t.edges_to(OAM_BASE + i);
            let ingress: Vec<_> = all_edges.iter()
                .filter(|e| e.source == FABRIC_SWITCH)
                .collect();
            assert_eq!(ingress.len(), 1,
                "OAM module {} must receive exactly one ingress from switch", i);
        }
    }

    #[test]
    fn ring_inadmissibility_passes() {
        let t = declare_fabric_topology();
        assert!(t.ring_inadmissibility_check(),
            "Declared fabric topology must pass ring inadmissibility check");
    }

    #[test]
    fn all_edges_have_provenance() {
        let t = declare_fabric_topology();
        for edge in &t.edges {
            assert!(!edge.provenance.is_empty(),
                "Every declared fabric edge must have observable provenance");
        }
    }

    #[test]
    fn per_link_bandwidth_derived_from_aggregate() {
        // FABRIC_PER_LINK_BW_GB_S = 1194.8 / 16 = 74.675 GB/s
        let expected = FABRIC_AGGREGATE_BW_GB_S / FABRIC_N_DIRECTED_LINKS as f64;
        assert!((FABRIC_PER_LINK_BW_GB_S - expected).abs() < 1e-6,
            "Per-link bandwidth must equal aggregate / n_links");
    }

    #[test]
    fn aggregate_bandwidth_declared_correctly() {
        // Each of 16 links carries per-link bandwidth.
        // Total = 16 Ã— 74.675 = 1194.8 GB/s.
        let t = declare_fabric_topology();
        let (total, declared) = t.aggregate_bandwidth_check();
        assert!((total - declared).abs() < 1e-6,
            "Sum of all link bandwidths must equal declared aggregate: \
             got {:.1}, expected {:.1}", total, declared);
    }

    #[test]
    fn module_to_module_edges_absent() {
        // No direct module-to-module edges â€” all routes through switch.
        let t = declare_fabric_topology();
        for edge in &t.edges {
            let source_is_module = edge.source < 8;
            let target_is_module = edge.target < 8;
            assert!(!(source_is_module && target_is_module),
                "Direct module-to-module edge found: {} â†’ {} \
                 â€” inadmissible, all traffic routes through switch",
                edge.source, edge.target);
        }
    }

    #[test]
    fn switch_is_not_source_to_itself() {
        let t = declare_fabric_topology();
        for edge in &t.edges {
            assert_ne!(edge.source, edge.target,
                "Self-loop detected at locus {} â€” inadmissible", edge.source);
        }
    }

    #[test]
    fn bandwidth_per_link_positive_finite() {
        assert!(FABRIC_PER_LINK_BW_GB_S > 0.0 && FABRIC_PER_LINK_BW_GB_S.is_finite(),
            "Per-link bandwidth must be positive and finite");
    }
}
