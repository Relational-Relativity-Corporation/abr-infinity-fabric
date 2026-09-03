# abr-infinity-fabric

**ABR Infinity Fabric — AMD Instinct MI355X topology declared as relational structure.**

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

---

## Current State — V0.3.0

**Phase 1: Structural Reduction declared and simulation-verified.**

75/75 tests passing. Verifier pass complete (2026-09-02).

V0.3.0 adds `fabric_sim.rs`: three workload dependency classes declared through M,
9-cell simulation matrix (3 classes × 3 working-set scales), predicted DF PMC beat
counts for hardware comparison via uProf MSR mode, and structural reduction factor
declared from topology. OC-IF-SIM-1 and OC-IF-SIM-2 open.

Verifier disposition (2026-09-02):
- Structurally/formally verified: simulation layer executing cleanly including
  provenance, topology, dependency-class, scaling, reduction-factor, and
  desktop-xGMI tests. Nine-cell matrix internally represented and tested.
- Not yet observationally verified: predicted CS traffic values. Hardware
  measurement columns declared and awaiting uProf MSR mode measurement:
  CS_READ_MEASURED, CS_WRITE_MEASURED, CCM_IN_MEASURED, XGMI_MEASURED,
  alongside V13 CPI and DRAM_PTI protocol.
- The 8× structural reduction is a topology-fixed prediction, not a hardware
  finding. Correspondence to hardware requires the declared measurement pass.

---

## Purpose

Declares the AMD Instinct MI355X Infinity Fabric as a directed relational graph
through M (AMD MI355X Platform specification, retrieved 2026-08-05) and applies
the ABR A operator to identify the declared lower-bandwidth locus and characterize
the hardware limits relevant to throughput.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric
hardware. OC-DB-3 remains open (bandwidth/working-set derivation retracted per
OC-IF-5).

---

## Phase 1 Simulation — Structural Reduction

### Declared Workload Classes

Three dependency classes declared through M:

- **Independent** — zero inter-region declared dependencies. Each data region
  is self-contained. Predicted CS beats: working set reads only. IF locus not
  activated.
- **WeaklyCoupled** — sparse inter-region declared dependencies (2 edges per
  region, 16 total at n_regions=8). Predicted CS beats: working set + 16
  dependency fetch beats.
- **FullyCoupled** — all-to-all declared dependencies (56 edges at n_regions=8).
  Predicted CS beats: working set + 56 dependency fetch beats.

### Declared Observables — DF PMC Events

Observable grounding: Section 7.1 "Fabric Performance Monitor Counter (PMC) Events",
Processor Programming Reference (PPR) for AMD Family 19h (Zen 4).
Source: Linux kernel patch commit 5b2ca349c313 (Sandipan Das, AMD, 2022-12-14).

Hardware measurement via uProf MSR mode on Ryzen 5 7600X:

| Event | Description | Unit |
|---|---|---|
| local_processor_read_data_beats_csN | CS read beats at CCD↔IOD interface | 64 bytes/beat |
| local_processor_write_data_beats_csN | CS write beats at CCD↔IOD interface | 64 bytes/beat |
| local_socket_inf0_inbound_data_beats_ccmN | CCM inbound beats (IOD internal) | 32 bytes/beat |
| local_socket_outbound_data_beats_linkN | xGMI outbound beats (expected zero on desktop) | 64 bytes/beat |

### Structural Reduction — Phase 1 Finding

The undeclared baseline — a stateless system with no dependency information —
treats every region as coupled to every other, generating n_regions² × ws_beats
CS read beats. Declared relational structure (Independent class) generates
n_regions × ws_beats. At n_regions=8: structural reduction factor = 8.0×,
fixed by topology independent of working-set scale.

This is a structural prediction from declared relational structure. It is not
a hardware finding. Hardware correspondence requires the declared measurement pass.

OC-IF-SIM-2 declares the interaction between this structural reduction and
quadratic attention complexity as the Phase 2 question: at transformer scale
(n tokens, O(n²) baseline), IF traffic scales as O(n²) for undeclared attention
and O(n·k) for declared relational attention where k is the admitted edge count.
The absolute beat savings grow quadratically with n.

### Running the Simulation

```
cargo run
cargo test
```

`cargo run` prints the full 9-cell comparison table with predicted beat counts
and hardware measurement columns. `cargo test` runs all 75 tests.

---

## Layer Convergence

| Layer | Repo | Finding |
|---|---|---|
| 1 | abr-grid-integration | grid physics → 100–175 MW viable band |
| 2 | abr-workload-architecture | compute → same band independently |
| 3 | abr-datacenter-build | rack declared; efficiency readable from ops |
| 4 | abr-infinity-fabric (this repo) | kernel traversal maps structurally to declared hardware topology; Phase 1 structural reduction established |

---

## Build and Run

```
cargo build --release
cargo test
cargo run
```

---

## Open Conditions

**OC-IF-SIM-1 OPEN** — Simulation predicts traffic at the IOD boundary (CS events)
on Ryzen 5 7600X. Correspondence to physical MI355X fabric traffic (inter-OAM
xGMI events) requires direct measurement on multi-die hardware. Scope of this
simulation-to-hardware comparison: declared relational structure → IOD memory
controller bandwidth, NOT inter-chiplet fabric traffic on MI355X.

**OC-IF-SIM-2 OPEN** — Structural reduction (Phase 1) declared and measurable at
any working-set scale. Interaction between structural reduction and quadratic
attention complexity not modeled in Phase 1. Phase 2 declares KV-cache traffic
as the working set and models the O(n²) → O(n·k) reduction at the fabric level.

**OC-IF-1** — Per-link bandwidth uniformity: AMD declares aggregate (1,194.8 GB/s).
Per-link derived assuming uniform distribution across 16 directed links. Replace
with direct per-link measurement when available.

**OC-IF-2** — Partition admissibility for inter-community workloads: closure
argument holds for independent community analyses only. Workloads with declared
inter-community dependencies require fabric bandwidth and a separate partition
derivation.

**OC-IF-3** — ROCm HIP kernel implementation: execution model declared here. HIP
kernel source is a downstream deliverable. Implementation must satisfy the declared
HipKernelSpec interface. Closes OC-IF-5 when direct MI355X measurement is obtained.

**OC-IF-4** — Single working-set read per pass assumption: declared for sparse
graphs where working set fits in cache. Formal derivation from operator mathematics
remains open.

**OC-IF-5** — Throughput figure is consistent with approximately constant per-edge
cost execution, not bandwidth-bound. The derivation in throughput_invariants.rs
assumed bandwidth-bound execution. Home system scaling measurement (abr-home-system-
benchmark, Ryzen 5 7600X, 2026-08-08) shows ns/edge approximately constant across
graph sizes 1,023–16,383 edges (3.4–3.8 ns/edge, two independent runs). Throughput
on MI355X requires revision: throughput = 1 / (n_edges × ns_per_edge_on_MI355X)
where ns_per_edge_on_MI355X must be determined by direct MI355X measurement.
The 7.6M analyses/second figure is a retracted upper bound. OC-IF-3 is the path
to closing this condition.

**OC-DB-3 OPEN** — Bandwidth/working-set throughput derivation retracted per OC-IF-5.
The 7.6M figure is a retracted upper bound, not a structural closure.

**OC-DB-6** — Self-describing property: supported by this repo execution model
declaration. Formal derivation from operator fixed-point remains open.

---

## Closed Open Conditions

**OC-DB-1 CLOSED** — Kernel-to-hardware mapping derived from declared structure.

---

## Version History

**V0.3.0 (2026-09-02):** Phase 1 structural reduction declared. fabric_sim.rs added:
three workload dependency classes, 9-cell simulation matrix (3 classes × 3 WS
scales), DF PMC beat predictions grounded in AMD PPR Section 7.1, undeclared
baseline declared, 8× structural reduction factor established from topology.
OC-IF-SIM-1 and OC-IF-SIM-2 opened. Verifier pass complete. 75/75 tests.

**V0.2.3 (2026-08-28):** F1–F4 closure under External Baseline Comparison criterion.
F1: throughput derivation retracted. F2: causal attribution downgraded to structural-
property language. F3: efficiency advantage language removed. F4: bottleneck →
lower-bandwidth locus. 57/57 tests.

**V0.1.0:** AMD MI355X Infinity Fabric declared as relational structure. OC-DB-1
closed. Four-layer convergence confirmed. 57/57 tests.

---

## Grounding Documents

- abr-datacenter-build V0.3.2 (OC-DB-7 retraction applied)
- abr-community-grid-match — Lompoc primary case, Verification PASS
- operators.rs V8 — ABR kernel declaration
- derived_invariants.rs V4.1 — Layer 3 invariants
- AMD MI355X Platform specification (retrieved 2026-08-05)
- AMD ROCm published specification
- AMD PPR Family 19h — Section 7.1 DF PMC events (Zen 4)
- abr-home-system-benchmark — V13 scaling measurement, Ryzen 5 7600X,
  CPI 6→46 across L2/L3/DRAM working-set regimes, 75/75 tests
  https://github.com/Relational-Relativity-Corporation/abr-home-system-benchmark
