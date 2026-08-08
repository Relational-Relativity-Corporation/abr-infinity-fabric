# abr-infinity-fabric

**ABR Infinity Fabric — AMD Instinct MI355X topology declared as relational structure.**

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware.
57/57 tests. Four-layer convergence confirmed.

## Purpose

Declares the AMD Instinct MI355X Infinity Fabric as a directed relational graph
through M (AMD MI355X Platform specification, retrieved 2026-08-05) and applies
the ABR A operator to identify the bandwidth bottleneck and derive throughput.

Closes OC-DB-1 and OC-DB-3 from abr-datacenter-build. See lib.rs for full
closure argument.

## Layer Convergence

- Layer 1  abr-grid-integration: grid physics -> 100-175 MW viable band
- Layer 2  abr-workload-architecture: compute -> same band independently
- Layer 3  abr-datacenter-build: rack declared; efficiency readable from ops
- Layer 4  abr-infinity-fabric (this repo): kernel maps to hardware natively

## Build and Run

cargo build --release
cargo test

## Closed Open Conditions

- OC-DB-1  CLOSED: kernel-to-hardware mapping derived from declared structure
- OC-DB-3  CLOSED STRUCTURALLY: throughput derived from declared constants

## Open Conditions

- OC-IF-1  Per-link bandwidth uniformity: AMD declares aggregate (1,194.8 GB/s).
           Per-link derived assuming uniform distribution across 16 directed links.
           Replace with direct per-link measurement when available.

- OC-IF-2  Partition admissibility for inter-community workloads: closure argument
           holds for independent community analyses only. Workloads with declared
           inter-community dependencies require fabric bandwidth and a separate
           partition derivation.

- OC-IF-3  ROCm HIP kernel implementation: execution model declared here.
           HIP kernel source is a downstream deliverable. Implementation must
           satisfy the declared HipKernelSpec interface.

- OC-IF-4  Single working-set read per pass assumption: declared for sparse graphs
           where working set fits in cache. Formal derivation from operator
           mathematics remains open.

- OC-IF-5  Throughput figure is latency-bound, not bandwidth-bound.
           The derivation in throughput_invariants.rs assumed bandwidth-bound
           execution (throughput = bandwidth / working_set). Scaling measurement
           on home system (abr-home-system-benchmark, Ryzen 5 7600X, 2026-08-08)
           confirms the ABR kernel is latency-bound by the B operator sequential
           dependency chain. NS/EDGE is constant across graph sizes 1,023-16,383
           edges (4.25-4.69 ns/edge, ratios 1.00-1.05). Throughput on MI355X
           requires revision from bandwidth/working-set derivation to:
             throughput = 1 / (n_edges x ns_per_edge_on_MI355X)
           where ns_per_edge_on_MI355X is determined by HBM3E memory latency,
           not bandwidth. The 7.6M analyses/second figure is an upper bound
           pending direct measurement on MI355X hardware. OC-IF-3 is the path
           to closing this condition.

- OC-DB-6  Self-describing property: supported by this repo execution model
           declaration. Formal derivation from operator fixed-point remains open.

## Grounding Documents

- abr-datacenter-build V0.2
- abr-community-grid-match — Lompoc primary case, Verification PASS
- operators.rs V8 — ABR kernel declaration
- derived_invariants.rs V4.1 — Layer 3 invariants
- AMD MI355X Platform specification (retrieved 2026-08-05)
- AMD ROCm published specification
- abr-home-system-benchmark — scaling measurement confirming latency-bound
  execution (2026-08-08, Ryzen 5 7600X, 18/18 tests, linear scaling confirmed)
