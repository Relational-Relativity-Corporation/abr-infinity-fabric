// lib.rs â€” Metatron Dynamics, Inc.
// abr-infinity-fabric: AMD Infinity Fabric declared as relational structure.
// Closes OC-DB-1 (kernel-to-hardware mapping) from abr-datacenter-build.
// Bounded over D. No claim beyond D.
//
// â”€â”€ Purpose â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// This repository formally closes OC-DB-1 from abr-datacenter-build:
//
//   OC-DB-1: The ABR operator traversal pattern on sparse declared graphs
//   is not yet formally mapped to AMD Infinity Fabric topology. The
//   efficiency advantage is structurally derived; the hardware-specific
//   throughput is an open condition.
//
// Closure argument:
//   1. Infinity Fabric declared as directed graph through M (AMD MI355X spec).
//   2. ABR A operator applied to fabric bandwidth field â€” sign consistent
//      with kernel V8, bottleneck identified as fabric switch (1,194.8 GB/s
//      vs module bandwidth 8,000 GB/s per module).
//   3. Community analysis working set (1 MB) fits entirely in HBM3E (288 GB).
//   4. Independent community analyses require zero inter-module communication.
//   5. ABR executes at full HBM3E bandwidth (8.0 TB/s per module) â€”
//      the fabric bottleneck is not active for this workload class.
//   6. Throughput derived structurally: ~7.6M analyses/second/module,
//      ~61M analyses/second at rack scale.
//
// â”€â”€ Declared Provenance Chain â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
//   Fabric topology    â†’ AMD MI355X Platform specification through M
//                        https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html
//   Bandwidth per link â†’ Derived from AMD declared aggregate (1,194.8 GB/s) Ã· 16 links
//   Community graph    â†’ abr-community-grid-match declared structure through M
//   Working set        â†’ abr-datacenter-build kernel_deployment.rs through M
//   Partition mapping  â†’ Derived from operator application, not assumed
//   Throughput figure  â†’ Derived from declared bandwidth and working set
//   Software stack     â†’ ROCm 7.0 / HIP â€” AMD published specification through M
//
// â”€â”€ Module Structure â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
//   fabric_topology     â€” Infinity Fabric as declared directed graph (9 loci, 16 edges)
//   fabric_field        â€” Bandwidth field; A operator; bottleneck identification
//   workload_graph      â€” Community graph as partitionable workload structure
//   partition_mapping   â€” Derives partition; checks AC-1 through AC-4; closes OC-DB-1
//   kernel_execution    â€” Execution model; ROCm HIP spec; efficiency basis
//   throughput_invariants â€” Structural throughput derivation; closes OC-DB-3
//   convergence         â€” Full chain integration test; formal OC-DB-1 closure
//
// â”€â”€ Open Conditions Closed â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// OC-DB-1  CLOSED â€” kernel-to-hardware mapping derived from declared structure.
//          ABR operators execute at HBM3E bandwidth for independent community
//          analyses. Zero fabric traffic. See convergence.rs for formal argument.
//
// OC-DB-3  CLOSED STRUCTURALLY â€” throughput derived from declared constants.
//          ~7.6M analyses/second per module; ~61M at rack scale.
//          Correspondence requires instrument measurement.
//
// â”€â”€ Open Conditions Remaining â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// OC-IF-1  Per-link bandwidth uniformity: AMD declares aggregate (1,194.8 GB/s).
//          Per-link derived assuming uniform distribution across 16 directed links.
//          Replace with direct per-link measurement when available.
//
// OC-IF-2  Partition admissibility for inter-community workloads: the closure
//          argument holds for INDEPENDENT community analyses. Workloads with
//          declared inter-community dependencies (e.g., regional optimization)
//          require fabric bandwidth and a separate partition derivation.
//
// OC-IF-3  ROCm HIP kernel implementation: the execution model is declared.
//          The HIP kernel source is a downstream deliverable. Implementation
//          must satisfy the declared HipKernelSpec interface.
//
// OC-DB-6  Self-describing property: supported by this repo's execution model
//          declaration (the kernel that justified Lompoc now maps to its
//          hardware). Formal derivation from operator fixed-point remains open.
//
// â”€â”€ Layer Convergence â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
// Layer 1  abr-grid-integration: grid physics â†’ 100-175 MW viable band.
// Layer 2  abr-workload-architecture: compute â†’ same band independently.
// Layer 3  abr-datacenter-build: rack declared; efficiency readable from ops.
// Layer 4  abr-infinity-fabric (this repo): kernel maps to hardware natively.
//
// Four independent derivations converging on the same declared structure.
//
// â”€â”€ Grounding Documents â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
//
//   abr-datacenter-build V0.2 â€” open conditions OC-DB-1 through OC-DB-6
//   abr-community-grid-match â€” Lompoc primary case, Verification PASS
//   operators.rs V8 â€” ABR kernel declaration
//   derived_invariants.rs V4.1 â€” Layer 3 invariants
//   AMD MI355X Platform specification (retrieved 2026-08-05)
//   AMD ROCm published specification

pub mod fabric_topology;
pub mod fabric_field;
pub mod workload_graph;
pub mod partition_mapping;
pub mod kernel_execution;
pub mod throughput_invariants;
pub mod convergence;
