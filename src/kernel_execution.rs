// kernel_execution.rs â€” Metatron Dynamics, Inc.
// ABR kernel execution model on declared Infinity Fabric topology.
// Bounded over D. No claim beyond D.
//
// â”€â”€ Declaration â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// This module declares the execution model for ABR operators on the
// MI355X platform. It does not contain HIP kernel source â€” that is a
// downstream deliverable (OC-IF-3).
//
// Declared software stack: ROCm 7.0 / HIP.
// Source: AMD ROCm published specification.
//   https://www.amd.com/en/products/software/rocm.html
// Observable provenance: AMD published software specification through M.
//
// â”€â”€ Execution Model â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
//   1. Each OAM module receives its assigned community analyses.
//   2. Each module executes A â†’ B â†’ R locally over its declared graphs.
//   3. Inter-module communication: zero for independent community analyses.
//   4. Results collected at the network egress boundary.
//
// â”€â”€ Why ABR Executes Efficiently on MI355X â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// The ABR kernel is memory-bandwidth-bound on sparse declared graphs.
// The working set (1 MB per community) fits entirely in HBM3E (288 GB).
// This means:
//
//   - No HBM3E eviction during A â†’ B â†’ R execution.
//   - Full 8.0 TB/s memory bandwidth available for operator traversal.
//   - No weight-loading bottleneck (ABR has no weight matrices).
//   - The sparse graph traversal pattern matches the Infinity Fabric
//     topology's declared structure: directed edges with provenance.
//
// Contrast with conventional LLM inference:
//   - LLM weights (140 GB for LLaMA 3 70B) continuously evict from HBM3E.
//   - Effective bandwidth is consumed by weight loading, not computation.
//   - The HBM3E bandwidth is the bottleneck for computation.
//
// For ABR: the operator traversal IS the computation. The working set
// remains resident. The full declared bandwidth is applied to relational
// graph traversal at every step of A â†’ B â†’ R.
//
// â”€â”€ Self-Describing Consequence (OC-DB-6 partial closure) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// The Lompoc facility hosts MI355X racks running this execution model.
// The community graph that identified Lompoc as viable (abr-community-grid-match)
// executes on the same hardware as a member of the Western US community queue.
// This repo (abr-infinity-fabric) formally maps the kernel to the hardware â€”
// closing the connection between the analytical framework and its substrate.
//
// OC-DB-6 partial closure: the self-describing property is now supported
// by a declared execution model rather than asserted as a boolean field.
// Full formal derivation from operator fixed-point remains open.
//
// â”€â”€ Open Conditions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// OC-IF-3: ROCm HIP kernel implementation â€” declared here, implemented
//          downstream. The HIP kernel must implement A â†’ B â†’ R as sparse
//          graph traversal on declared edges, matching this execution model.

/// Declared software stack for ABR kernel execution.
/// Source: AMD ROCm published specification through M.
pub const ROCM_SOFTWARE_STACK: &str = "ROCm 7.0 / HIP";

/// Declared execution model for ABR operators on MI355X.
#[derive(Debug, Clone)]
pub struct ExecutionModel {
    /// Number of OAM modules in the declared execution environment.
    pub n_modules: usize,
    /// Declared software stack.
    pub software_stack: &'static str,
    /// Declared inter-module protocol for independent analyses.
    pub inter_module_protocol: &'static str,
    /// Whether inter-module communication is required for declared workload.
    pub inter_module_required: bool,
    /// Declared memory bandwidth per module. Units: GB/s.
    pub memory_bandwidth_per_module_gb_s: f64,
    /// Declared working set per community analysis. Units: bytes.
    pub community_working_set_bytes: u64,
    /// Whether working set fits in HBM3E without eviction.
    pub working_set_resident: bool,
    /// Open condition for HIP implementation.
    pub open_condition: &'static str,
}

/// Specification for the downstream HIP kernel implementation.
/// This is the declared interface that the HIP kernel must satisfy.
/// OC-IF-3: implementation is a downstream deliverable.
#[derive(Debug, Clone)]
pub struct HipKernelSpec {
    /// Kernel name.
    pub name: &'static str,
    /// ABR operator this kernel implements.
    pub operator: &'static str,
    /// Input type declaration.
    pub input_declaration: &'static str,
    /// Output type declaration.
    pub output_declaration: &'static str,
    /// Memory access pattern â€” critical for efficiency claim.
    pub memory_access_pattern: &'static str,
    /// Whether this kernel requires inter-block communication.
    pub requires_inter_block_comm: bool,
}

/// Returns the declared ABR execution model for MI355X.
/// All values trace to declared specifications through M.
pub fn declare_execution_model() -> ExecutionModel {
    use crate::workload_graph::{COMMUNITY_WORKING_SET_BYTES, HBM3E_BYTES_PER_MODULE,
                                 N_OAM_MODULES};
    use crate::fabric_field::OAM_MEMORY_BW_GB_S;

    ExecutionModel {
        n_modules: N_OAM_MODULES,
        software_stack: ROCM_SOFTWARE_STACK,
        inter_module_protocol: "None required â€” independent community analyses \
                                 execute as separate declared Ds with zero \
                                 inter-module communication",
        inter_module_required: false,
        memory_bandwidth_per_module_gb_s: OAM_MEMORY_BW_GB_S,
        community_working_set_bytes: COMMUNITY_WORKING_SET_BYTES,
        // 1 MB working set << 288 GB HBM3E â€” resident without eviction.
        working_set_resident: COMMUNITY_WORKING_SET_BYTES < HBM3E_BYTES_PER_MODULE,
        open_condition: "OC-IF-3: ROCm HIP kernel implementation is a \
                         downstream deliverable. This model declares the \
                         interface; implementation must match.",
    }
}

/// Returns the declared HIP kernel specifications for A, B, and R operators.
/// OC-IF-3: these specs are the interface contract for downstream implementation.
pub fn declare_hip_kernel_specs() -> Vec<HipKernelSpec> {
    vec![
        HipKernelSpec {
            name: "abr_operator_a",
            operator: "A",
            input_declaration: "NodeField: declared observable values at each locus; \
                                DeclaredRelations: directed edge list with provenance",
            output_declaration: "EdgeField: directed differences x[source] - x[target] \
                                  at each declared edge",
            memory_access_pattern: "Sparse gather: reads source and target locus values \
                                     for each declared edge. Working set = node field + \
                                     edge list. Resident in HBM3E without eviction.",
            requires_inter_block_comm: false,
        },
        HipKernelSpec {
            name: "abr_operator_b",
            operator: "B",
            input_declaration: "EdgeField: A operator output; \
                                DeclaredRelations: successor edge indices",
            output_declaration: "EdgeField: accumulated values \
                                  g[e] + sum(g[succ(e)])",
            memory_access_pattern: "Sparse gather over successor sets. \
                                     Terminal edges accumulate nothing. \
                                     No wraparound. Resident in HBM3E.",
            requires_inter_block_comm: false,
        },
        HipKernelSpec {
            name: "abr_operator_r",
            operator: "R",
            input_declaration: "EdgeField: B operator output; \
                                Vec<f64>: rho values per node; \
                                DeclaredRelations: successor and predecessor sets",
            output_declaration: "EdgeField: g[e] + rho[src(e)] * \
                                  (sum_succ - sum_pred)",
            memory_access_pattern: "Sparse gather over successor and predecessor sets. \
                                     Antisymmetric circulation applied locally. \
                                     Resident in HBM3E.",
            requires_inter_block_comm: false,
        },
    ]
}

/// Declared efficiency advantage at this execution model.
/// Compares ABR working-set-resident execution to reference LLM inference.
///
/// Source for reference LLM: declared in abr-datacenter-build declared_hardware.rs
///   REFERENCE_LLM_WEIGHT_BYTES = 140 GB (Meta LLaMA 3 70B, FP16)
///   Source: Meta LLaMA 3 model card through M.
///
/// This is a declared structural comparison, not a measured throughput.
/// OC-DB-1 formal closure enables throughput derivation in throughput_invariants.rs.
pub fn declared_execution_advantage() -> f64 {
    use crate::workload_graph::COMMUNITY_WORKING_SET_BYTES;

    // Reference LLM weight load â€” from abr-datacenter-build declaration.
    // LLaMA 3 70B at FP16: 140 GB = 140e9 bytes.
    let reference_llm_weight_bytes: f64 = 140.0e9;

    // ABR working set: 1 MB per community (declared upper bound).
    let abr_working_set: f64 = COMMUNITY_WORKING_SET_BYTES as f64;

    // Ratio: reference LLM weight / ABR working set.
    // Higher ratio = larger efficiency advantage for ABR.
    reference_llm_weight_bytes / abr_working_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_model_declared() {
        let model = declare_execution_model();
        assert_eq!(model.n_modules, 8,
            "Execution model must declare 8 OAM modules");
        assert!(model.software_stack.contains("ROCm"),
            "Software stack must declare ROCm");
    }

    #[test]
    fn working_set_resident_in_hbm3e() {
        let model = declare_execution_model();
        assert!(model.working_set_resident,
            "Community working set must be resident in HBM3E without eviction");
    }

    #[test]
    fn inter_module_not_required() {
        let model = declare_execution_model();
        assert!(!model.inter_module_required,
            "Independent community analyses must not require inter-module communication");
    }

    #[test]
    fn hip_kernel_specs_cover_all_operators() {
        let specs = declare_hip_kernel_specs();
        assert_eq!(specs.len(), 3,
            "Must declare HIP specs for all three operators: A, B, R");
        let operators: Vec<&str> = specs.iter().map(|s| s.operator).collect();
        assert!(operators.contains(&"A"), "Must declare HIP spec for A");
        assert!(operators.contains(&"B"), "Must declare HIP spec for B");
        assert!(operators.contains(&"R"), "Must declare HIP spec for R");
    }

    #[test]
    fn no_kernel_requires_inter_block_comm() {
        // ABR sparse graph traversal is local â€” no global synchronization.
        // Each wavefront processes its declared edge set independently.
        let specs = declare_hip_kernel_specs();
        for spec in &specs {
            assert!(!spec.requires_inter_block_comm,
                "Operator {} must not require inter-block communication \
                 â€” ABR traversal is local", spec.operator);
        }
    }

    #[test]
    fn execution_advantage_in_declared_range() {
        // 140 GB LLM weights / 1 MB ABR working set = 140,000x
        let advantage = declared_execution_advantage();
        assert!(advantage > 100_000.0,
            "Declared execution advantage must exceed 100,000x");
        assert!(advantage < 200_000.0,
            "Declared execution advantage must be below 200,000x \
             (confirms community working set, not rack topology, is used)");
    }

    #[test]
    fn open_condition_if3_named() {
        let model = declare_execution_model();
        assert!(model.open_condition.contains("OC-IF-3"),
            "Execution model must name OC-IF-3 as open condition");
    }

    #[test]
    fn memory_bandwidth_declared_from_specification() {
        let model = declare_execution_model();
        assert!((model.memory_bandwidth_per_module_gb_s - 8_000.0).abs() < 1e-6,
            "Memory bandwidth must be 8,000 GB/s (8.0 TB/s from AMD specification)");
    }
}
