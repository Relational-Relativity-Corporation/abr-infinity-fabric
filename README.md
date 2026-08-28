# abr-infinity-fabric

**ABR Infinity Fabric — AMD Instinct MI355X topology declared as relational structure.**

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware.
57/57 tests. Four-layer convergence confirmed.

**V0.2 (2026-08-28):** Findings F1–F4 from Verifier re-review under External
Baseline Comparison criterion (operators.rs V8, 2026-08-28). F1: bandwidth-bound
throughput derivation retracted as OC-DB-3 closure (OC-IF-5 already noted the
derivation model was wrong; this pass propagates that finding through convergence
and tests). F2: "latency-bound by B operator" downgraded to "consistent with
approximately constant per-edge cost" — the observation does not isolate the
mechanism. F3: "efficiency advantage" language replaced with structural-property
language throughout; cross-task working-set comparison retracted per OC-DB-7 in
abr-datacenter-build. F4: "bottleneck" → "lower-bandwidth locus" where no
workload demand is established.

## Purpose

Declares the AMD Instinct MI355X Infinity Fabric as a directed relational graph
through M (AMD MI355X Platform specification, retrieved 2026-08-05) and applies
the ABR A operator to identify the declared lower-bandwidth locus and characterize
the hardware limits relevant to throughput.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware.
OC-DB-3 remains open (bandwidth/working-set derivation retracted per OC-IF-5).

## Layer Convergence

- Layer 1  abr-grid-integration: grid physics -> 100-175 MW viable band
- Layer 2  abr-workload-architecture: compute -> same band independently
- Layer 3  abr-datacenter-build: rack declared; efficiency readable from ops
- Layer 4  abr-infinity-fabric (this repo): kernel traversal maps structurally
           to the declared hardware topology

## Build and Run

cargo build --release
cargo test

## Closed Open Conditions

- OC-DB-1  CLOSED: kernel-to-hardware mapping derived from declared structure
- OC-DB-3  OPEN — bandwidth/working-set throughput derivation retracted per OC-IF-5.
           The 7.6M figure is a retracted upper bound, not a structural closure.
           Actual throughput requires direct MI355X measurement.

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
           satisfy the declared HipKernelSpec interface. Closes OC-IF-5 when
           direct MI355X measurement is obtained.

- OC-IF-4  Single working-set read per pass assumption: declared for sparse graphs
           where working set fits in cache. Formal derivation from operator
           mathematics remains open.

- OC-IF-5  Throughput figure is consistent with approximately constant per-edge
           cost execution, not bandwidth-bound.
           The derivation in throughput_invariants.rs assumed bandwidth-bound
           execution (throughput = bandwidth / working_set). Scaling measurement
           on home system (abr-home-system-benchmark, Ryzen 5 7600X, 2026-08-08,
           24/24 tests, two independent runs) shows NS/EDGE approximately constant
           across graph sizes 1,023-16,383 edges (3.4-3.8 ns/edge across both
           runs, ratios 0.956-1.072). This result is consistent with approximately
           constant per-edge cost for V7 ABR operators on this hardware and
           declared open-chain topology. Throughput on MI355X requires revision
           from bandwidth/working-set derivation to:
             throughput = 1 / (n_edges x ns_per_edge_on_MI355X)
           where ns_per_edge_on_MI355X must be determined by direct MI355X
           measurement; the home-system scaling result indicates that per-edge
           execution cost, rather than aggregate bandwidth alone, must be
           represented in the throughput model. The
           7.6M analyses/second figure is a retracted upper bound pending direct
           measurement on MI355X hardware. OC-IF-3 is the path to closing
           this condition.

- OC-DB-6  Self-describing property: supported by this repo execution model
           declaration. Formal derivation from operator fixed-point remains open.

## Grounding Documents

- abr-datacenter-build V0.3.2 (OC-DB-7 retraction applied)
- abr-community-grid-match -- Lompoc primary case, Verification PASS
- operators.rs V7 -- ABR kernel declaration (ABR formulas lines 890-988)
- derived_invariants.rs V4.1 -- Layer 3 invariants
- AMD MI355X Platform specification (retrieved 2026-08-05)
- AMD ROCm published specification
- abr-home-system-benchmark -- scaling measurement consistent with
  approximately constant per-edge cost for V7 ABR operators
  (2026-08-08, Ryzen 5 7600X, 24/24 tests, two independent runs,
  3.4-3.8 ns/edge confirmed)
  https://github.com/Relational-Relativity-Corporation/abr-home-system-benchmark
