// lib.rs -- Metatron Dynamics, Inc.
// abr-infinity-fabric: AMD Infinity Fabric declared as relational structure.
// Closes OC-DB-1 (kernel-to-hardware mapping) from abr-datacenter-build.
// Bounded over D. No claim beyond D.
//
// -- Purpose ------------------------------------------------------------------
//
// This repository formally closes OC-DB-1 from abr-datacenter-build:
//
//   OC-DB-1: The ABR operator traversal pattern on sparse declared graphs
//   is not yet formally mapped to AMD Infinity Fabric topology. The
//   structural mapping is derived; the hardware-specific throughput is an
//   open condition. No ABR-specific efficiency advantage is claimed
//   without a matched external baseline (External Baseline Comparison
//   criterion, operators.rs V8, 2026-08-28).
//
// Closure argument:
//   1. Infinity Fabric declared as directed graph through M (AMD MI355X spec).
//   2. ABR A operator applied to fabric bandwidth field -- sign consistent
//      with kernel V8, lower-bandwidth locus identified as fabric switch (1,194.8 GB/s
//      vs module bandwidth 8,000 GB/s per module).
//   3. Community analysis working set (1 MB) fits entirely in HBM3E (288 GB).
//   4. Independent community analyses require zero inter-module communication.
//   5. Independent community analyses require zero inter-module communication
//      -- the fabric lower-bandwidth locus is not active for this workload class.
//   6. Home-system measurements (abr-home-system-benchmark, Ryzen 5 7600X,
//      2026-08-08) are consistent with approximately constant per-edge
//      execution cost across 1,023-16,383 edges (4.25-4.69 ns/edge,
//      ratios 1.00-1.05). The measurement does not isolate the hardware
//      or operator mechanism producing that cost.
//   7. Throughput requires revision per OC-IF-5. Admissible derivation:
//      throughput = 1 / (n_edges x ns_per_edge_on_MI355X).
//      7.6M analyses/second is an upper bound pending direct measurement
//      on MI355X hardware. OC-IF-3 is the path to closing this condition.
//
// -- Declared Provenance Chain ------------------------------------------------
//
//   Fabric topology    -> AMD MI355X Platform specification through M
//                         https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html
//   Bandwidth per link -> Derived from AMD declared aggregate (1,194.8 GB/s) / 16 links
//   Community graph    -> abr-community-grid-match declared structure through M
//   Working set        -> abr-datacenter-build kernel_deployment.rs through M
//   Partition mapping  -> Derived from operator application, not assumed
//   Throughput figure  -> Revised per OC-IF-5 (see below)
//   Software stack     -> ROCm 7.0 / HIP -- AMD published specification through M
//
// -- Module Structure ---------------------------------------------------------
//
//   fabric_topology     -- Infinity Fabric as declared directed graph (9 loci, 16 edges)
//   fabric_field        -- Bandwidth field; A operator; lower-bandwidth locus identification
//   workload_graph      -- Community graph as partitionable workload structure
//   partition_mapping   -- Derives partition; checks AC-1 through AC-4; closes OC-DB-1
//   kernel_execution    -- Execution model; ROCm HIP spec; efficiency basis
//   throughput_invariants -- Retracted bandwidth-bound upper bound (OC-DB-3 OPEN)
//   convergence         -- Full chain integration test; formal OC-DB-1 closure
//   fabric_sim          -- Dependency class simulation; structural reduction (V0.3.0)
//   sim3a               -- OC-IF-SIM-3A: relational topology scaling correspondence
//
// -- Open Conditions Closed ---------------------------------------------------
//
// OC-DB-1  CLOSED -- kernel-to-hardware mapping derived from declared structure.
//          ABR operators execute at HBM3E bandwidth for independent community
//          analyses. Zero fabric traffic. See convergence.rs for formal argument.
//
// OC-DB-3  OPEN -- bandwidth/working-set throughput derivation retracted per
//          OC-IF-5. The 7.6M figure is a retracted upper bound, not a closure.
//          Actual throughput requires direct MI355X measurement.
//
// -- Open Conditions Remaining ------------------------------------------------
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
// OC-IF-4  Single working-set read per pass assumption: declared for sparse
//          graphs where working set fits in cache. Formal derivation from
//          operator mathematics remains open.
//
// OC-IF-5  Throughput figure is consistent with approximately constant per-edge
//          The derivation in throughput_invariants.rs assumed bandwidth-bound
//          execution (throughput = bandwidth / working_set). Scaling measurement
//          on home system (abr-home-system-benchmark, Ryzen 5 7600X, 2026-08-08,
//          18/18 tests) is consistent with approximately constant per-edge
//          execution cost across graph sizes 1,023-16,383 edges (4.25-4.69
//          ns/edge, ratios 1.00-1.05). The measurement does not isolate the
//          hardware or operator mechanism producing that cost.
//          Throughput on MI355X requires revision from bandwidth/working-set
//          derivation to a per-edge cost model:
//            T_analysis = n_edges * t_edge_MI355X
//          where t_edge_MI355X must be determined by direct MI355X measurement.
//          The 7.6M analyses/second figure is a retracted upper bound.
//          OC-IF-3 (HIP implementation) is the path to closing this condition.
//
// OC-DB-6  Self-describing property: supported by this repo's execution model
//          declaration (the kernel that justified Lompoc now maps to its
//          hardware). Formal derivation from operator fixed-point remains open.
//
// OC-IF-SIM-1  OPEN -- DF PMC beat counts require Linux + AMDuProf MSR mode.
//
// OC-IF-SIM-2  OPEN -- O(n^2) complexity interaction with structural reduction
//              declared for Phase 2. At transformer scale (n tokens, quadratic
//              baseline = n^2 pairwise ops), IF traffic scales as O(n^2) for
//              undeclared attention and O(n*k) for declared relational attention.
//              Absolute beat savings grow quadratically with n.
//
// OC-IF-SIM-3A CLOSED -- Relational topology scaling correspondence (2026-09-05).
//              Derived ratio 8.0000 (from declared topology) falls within observed
//              hardware range [7.945, 8.023]. Peak-to-peak spread 0.078 = 0.975%
//              of derived ratio — within documented hardware noise floor (~2%) for
//              complex processor substrates (SideRand arXiv:1810.00567, Robust
//              Benchmarking arXiv:1608.04295, PassMark benchmarking forum 2018).
//              The derived ratio is the structural fixed point; observations are
//              consistent with bounded physical noise around it.
//              See sim3a.rs, evaluate_correspondence(), 87/87 tests passing.
//
// OC-IF-SIM-3B OPEN -- Absolute hardware cost gradient correspondence.
//              Separate from 3A. Non-blocking.
//
// -- Layer Convergence --------------------------------------------------------
//
// Layer 1  abr-grid-integration: grid physics -> 100-175 MW viable band.
// Layer 2  abr-workload-architecture: compute -> same band independently.
// Layer 3  abr-datacenter-build: rack declared; efficiency readable from ops.
// Layer 4  abr-infinity-fabric (this repo): kernel maps to hardware natively.
//
// Four independent derivations converging on the same declared structure.
//
// -- Grounding Documents ------------------------------------------------------
//
//   abr-datacenter-build V0.2 -- open conditions OC-DB-1 through OC-DB-6
//   abr-community-grid-match -- Lompoc primary case, Verification PASS
//   operators.rs V8 -- ABR kernel declaration
//   derived_invariants.rs V4.1 -- Layer 3 invariants
//   AMD MI355X Platform specification (retrieved 2026-08-05)
//   AMD ROCm published specification
//   abr-home-system-benchmark V13 -- chain_only Pass A: CPI and DRAM_PTI
//     scaling across 4/32/128 MB working-set scales (2026-09-01, Ryzen 5 7600X)
//   bench_dependency_classes.rs -- structural ordering correspondence
//     (2026-09-03, Ryzen 5 7600X): FC/Ind ratio 7.945-8.023 stable across
//     107x absolute cost change
//   OC-IF-SIM-3.md -- open condition declaration with Origin confirmations

pub mod fabric_topology;
pub mod fabric_field;
pub mod workload_graph;
pub mod partition_mapping;
pub mod kernel_execution;
pub mod throughput_invariants;
pub mod convergence;

pub mod fabric_sim;
pub mod sim3a;
