// workload_graph.rs â€” Metatron Dynamics, Inc.
// Community analysis graph declared as partitionable workload structure.
// Bounded over D. No claim beyond D.
//
// â”€â”€ Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// A community analysis graph is a declared relational structure from
// abr-community-grid-match. It has declared loci (observable variables)
// and declared edges (relational dependencies between observables).
//
// Source for working set declaration:
//   abr-datacenter-build kernel_deployment.rs â€” COMMUNITY_WORKING_SET_BYTES = 1 MB
//   abr-community-grid-match declared observable structure (~50 observables)
//   Observable provenance: declared observable structure through M.
//
// â”€â”€ Partition Structure â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// For execution on 8 OAM modules, the community graph must be decomposable
// into subgraphs (partitions) such that:
//
//   AC-1: Each partition's working set fits within one module's HBM3E.
//   AC-2: Inter-partition edges correspond to declared Infinity Fabric links.
//   AC-3: No partition requires undeclared inter-module communication.
//   AC-4: Operator output is consistent across partition boundaries.
//
// â”€â”€ Working Set Analysis â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Declared community working set: 1 MB (generous upper bound).
// HBM3E per module: 288 GB = 309,237,645,312 bytes.
// Communities per module: 288 GB / 1 MB = 294,912.
//
// At this ratio, partition granularity is not the constraint for single
// community analysis. The entire community graph fits in HBM3E with
// enormous headroom. Partitioning is relevant for parallel multi-community
// execution across modules.
//
// â”€â”€ Declared Graph Structure â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Community observables declared through M (from abr-community-grid-match):
//   - Grid reliability metrics (SAIDI, CAIDI)
//   - Demand anchor observables (facility demand, employment, fiscal impact)
//   - Thermal demand observables (community heat absorption capacity)
//   - Infrastructure observables (grid capacity, transmission constraints)
//
// Declared edge types:
//   - Spatial continuation: within-category observable dependencies
//   - Coupling: cross-category relational dependencies
//   - Persistence: evolution across declared measurement steps
//
// OC-IF-2: Whether the community graph partitions cleanly onto 8 modules
//   without inter-partition edge overflow is not yet derived. This module
//   declares the graph structure; partition_mapping.rs derives the assignment.

/// Declared working set per community analysis.
/// Source: abr-datacenter-build kernel_deployment.rs
///   COMMUNITY_WORKING_SET_BYTES = 1,048,576 (1 MB generous upper bound)
/// Observable provenance: declared observable structure from
///   abr-community-grid-match through M.
/// Units: bytes.
pub const COMMUNITY_WORKING_SET_BYTES: u64 = 1_048_576; // 1 MB

/// HBM3E capacity per module in bytes.
/// Source: AMD MI355X Platform specification â€” 288 GB per module.
/// Observable provenance: AMD published platform specification through M.
pub const HBM3E_BYTES_PER_MODULE: u64 = 288 * 1_073_741_824; // 288 GB

/// Number of OAM modules available for workload partition.
/// Source: AMD MI355X Platform specification â€” 8 OAM modules per platform.
pub const N_OAM_MODULES: usize = 8;

/// Declared community graph locus types.
/// Each type corresponds to a category of declared observable through M.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunityLocus {
    /// Grid reliability observable (SAIDI, CAIDI, outage frequency).
    GridReliability,
    /// Demand anchor observable (facility demand, employment, fiscal impact).
    DemandAnchor,
    /// Thermal demand observable (community heat absorption capacity).
    ThermalDemand,
    /// Infrastructure observable (grid capacity, transmission constraints).
    Infrastructure,
}

/// Declared community graph edge types.
/// Each type has declared observable provenance through M.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunityEdgeType {
    /// Within-category observable dependency.
    /// Observable: declared within abr-community-grid-match M mapping.
    SpatialContinuation,
    /// Cross-category relational dependency.
    /// Observable: declared coupling between observable categories through M.
    Coupling,
    /// Evolution across declared measurement steps.
    /// Observable: declared sequence of observations through M.
    Persistence,
}

/// A declared locus in the community analysis graph.
#[derive(Debug, Clone)]
pub struct CommunityGraphLocus {
    /// Locus index.
    pub index: usize,
    /// Locus type â€” determines which observable category it represents.
    pub locus_type: CommunityLocus,
    /// Observable name declared through M.
    pub observable_name: &'static str,
}

/// A declared edge in the community analysis graph.
#[derive(Debug, Clone)]
pub struct CommunityGraphEdge {
    /// Source locus index.
    pub source: usize,
    /// Target locus index.
    pub target: usize,
    /// Edge type â€” determines observable provenance class.
    pub edge_type: CommunityEdgeType,
    /// Observable property establishing this direction through M.
    pub provenance: &'static str,
}

/// A declared community analysis graph.
/// Represents one community's declared relational structure.
#[derive(Debug, Clone)]
pub struct CommunityWorkloadGraph {
    /// Community name.
    pub name: &'static str,
    /// Declared loci â€” observable variables.
    pub loci: Vec<CommunityGraphLocus>,
    /// Declared edges â€” relational dependencies.
    pub edges: Vec<CommunityGraphEdge>,
    /// Working set in bytes â€” must satisfy AC-1.
    pub working_set_bytes: u64,
}

impl CommunityWorkloadGraph {
    /// Number of declared loci.
    pub fn n_loci(&self) -> usize {
        self.loci.len()
    }

    /// Number of declared edges.
    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }

    /// Admissibility check AC-1: working set fits within one module's HBM3E.
    pub fn fits_in_module(&self) -> bool {
        self.working_set_bytes <= HBM3E_BYTES_PER_MODULE
    }

    /// How many copies of this graph fit simultaneously in one module.
    pub fn simultaneous_per_module(&self) -> u64 {
        if self.working_set_bytes == 0 { return 0; }
        HBM3E_BYTES_PER_MODULE / self.working_set_bytes
    }

    /// Total simultaneous analyses across all 8 modules.
    pub fn total_simultaneous_rack(&self) -> u64 {
        self.simultaneous_per_module() * N_OAM_MODULES as u64
    }
}

/// Declares the Lompoc community analysis graph structure.
/// Source: abr-community-grid-match declared observable structure.
/// Observable provenance: declared M mapping from abr-community-grid-match.
///
/// This is the primary declared case (Verification PASS).
/// Other communities in the Western US queue follow the same graph structure
/// with different observable values â€” same topology, different field values.
pub fn declare_lompoc_community_graph() -> CommunityWorkloadGraph {
    let loci = vec![
        // Grid reliability observables
        CommunityGraphLocus {
            index: 0,
            locus_type: CommunityLocus::GridReliability,
            observable_name: "SAIDI_minutes_per_customer_year",
        },
        CommunityGraphLocus {
            index: 1,
            locus_type: CommunityLocus::GridReliability,
            observable_name: "CAIDI_minutes_per_interruption",
        },
        // Demand anchor observables
        CommunityGraphLocus {
            index: 2,
            locus_type: CommunityLocus::DemandAnchor,
            observable_name: "vandenberg_processing_budget_annual_dollars",
        },
        CommunityGraphLocus {
            index: 3,
            locus_type: CommunityLocus::DemandAnchor,
            observable_name: "agricultural_sector_demand_mw",
        },
        CommunityGraphLocus {
            index: 4,
            locus_type: CommunityLocus::DemandAnchor,
            observable_name: "municipal_tax_revenue_per_facility_dollar",
        },
        // Thermal demand observables
        CommunityGraphLocus {
            index: 5,
            locus_type: CommunityLocus::ThermalDemand,
            observable_name: "community_heat_absorption_capacity_mw",
        },
        // Infrastructure observables
        CommunityGraphLocus {
            index: 6,
            locus_type: CommunityLocus::Infrastructure,
            observable_name: "grid_capacity_mw",
        },
        CommunityGraphLocus {
            index: 7,
            locus_type: CommunityLocus::Infrastructure,
            observable_name: "transmission_constraint_mw",
        },
    ];

    let edges = vec![
        // Grid reliability â†’ demand anchor coupling
        CommunityGraphEdge {
            source: 0, target: 2,
            edge_type: CommunityEdgeType::Coupling,
            provenance: "Grid reliability drives demand anchor viability: \
                         SAIDI reduction correlates with anchor stability \
                         â€” declared from abr-community-grid-match through M",
        },
        // Demand anchor â†’ infrastructure coupling
        CommunityGraphEdge {
            source: 2, target: 6,
            edge_type: CommunityEdgeType::Coupling,
            provenance: "Demand anchor demand against grid capacity: \
                         declared load relationship through M",
        },
        CommunityGraphEdge {
            source: 3, target: 6,
            edge_type: CommunityEdgeType::Coupling,
            provenance: "Agricultural sector demand against grid capacity: \
                         declared load relationship through M",
        },
        // Infrastructure â†’ thermal coupling
        CommunityGraphEdge {
            source: 6, target: 5,
            edge_type: CommunityEdgeType::Coupling,
            provenance: "Grid capacity determines viable facility size, \
                         which determines thermal output â€” declared through M",
        },
        // Infrastructure spatial continuation
        CommunityGraphEdge {
            source: 6, target: 7,
            edge_type: CommunityEdgeType::SpatialContinuation,
            provenance: "Grid capacity to transmission constraint: \
                         within-infrastructure observable dependency through M",
        },
        // Fiscal observable
        CommunityGraphEdge {
            source: 4, target: 2,
            edge_type: CommunityEdgeType::Coupling,
            provenance: "Tax revenue per facility dollar grounds demand anchor \
                         economic viability â€” declared through M",
        },
    ];

    CommunityWorkloadGraph {
        name: "Lompoc_CA",
        loci,
        edges,
        working_set_bytes: COMMUNITY_WORKING_SET_BYTES,
    }
}

/// A partition of a community graph assigned to one OAM module.
#[derive(Debug, Clone)]
pub struct GraphPartition {
    /// OAM module index this partition is assigned to.
    pub module_index: usize,
    /// Locus indices in this partition.
    pub locus_indices: Vec<usize>,
    /// Edges entirely within this partition (intra-partition).
    pub intra_edges: Vec<CommunityGraphEdge>,
    /// Edges crossing to other partitions (inter-partition â€” must map to fabric links).
    pub inter_edges: Vec<CommunityGraphEdge>,
    /// Working set for this partition. Units: bytes.
    pub partition_working_set_bytes: u64,
}

impl GraphPartition {
    /// AC-1 check: partition working set fits in module HBM3E.
    pub fn fits_in_module(&self) -> bool {
        self.partition_working_set_bytes <= HBM3E_BYTES_PER_MODULE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lompoc_graph_declared() {
        let g = declare_lompoc_community_graph();
        assert_eq!(g.name, "Lompoc_CA");
        assert!(g.n_loci() > 0, "Graph must have declared loci");
        assert!(g.n_edges() > 0, "Graph must have declared edges");
    }

    #[test]
    fn working_set_fits_in_module() {
        let g = declare_lompoc_community_graph();
        assert!(g.fits_in_module(),
            "Community working set must fit within declared HBM3E per module");
    }

    #[test]
    fn simultaneous_analyses_large() {
        let g = declare_lompoc_community_graph();
        assert!(g.simultaneous_per_module() > 200_000,
            "Each module must support >200,000 simultaneous community analyses");
    }

    #[test]
    fn total_simultaneous_rack_scale() {
        let g = declare_lompoc_community_graph();
        assert!(g.total_simultaneous_rack() > 1_000_000,
            "Full rack must support >1M simultaneous community analyses");
    }

    #[test]
    fn all_edges_have_provenance() {
        let g = declare_lompoc_community_graph();
        for edge in &g.edges {
            assert!(!edge.provenance.is_empty(),
                "Every declared community graph edge must have provenance");
        }
    }

    #[test]
    fn working_set_matches_declared_constant() {
        let g = declare_lompoc_community_graph();
        assert_eq!(g.working_set_bytes, COMMUNITY_WORKING_SET_BYTES,
            "Graph working set must match declared constant");
    }

    #[test]
    fn hbm3e_capacity_declared_correctly() {
        // 288 GB = 288 Ã— 1,073,741,824 bytes
        let expected: u64 = 288 * 1_073_741_824;
        assert_eq!(HBM3E_BYTES_PER_MODULE, expected,
            "HBM3E capacity must be 288 GB from AMD specification");
    }

    #[test]
    fn locus_types_cover_declared_observables() {
        let g = declare_lompoc_community_graph();
        let has_grid = g.loci.iter().any(|l| l.locus_type == CommunityLocus::GridReliability);
        let has_demand = g.loci.iter().any(|l| l.locus_type == CommunityLocus::DemandAnchor);
        let has_thermal = g.loci.iter().any(|l| l.locus_type == CommunityLocus::ThermalDemand);
        let has_infra = g.loci.iter().any(|l| l.locus_type == CommunityLocus::Infrastructure);
        assert!(has_grid && has_demand && has_thermal && has_infra,
            "Community graph must declare all four observable categories");
    }
}
