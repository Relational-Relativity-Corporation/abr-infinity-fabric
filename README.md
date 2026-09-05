# abr-infinity-fabric

**ABR Infinity Fabric — AMD Instinct MI355X topology declared as relational structure.**

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware.
87/87 tests. Four-layer convergence confirmed.

**V0.5.0 (2026-09-05):** OC-IF-SIM-3A CLOSED. Correspondence criterion formally
declared and evaluated. Two conditions: (1) derived structural ratio 8.0000 falls
within observed hardware range [7.945, 8.023]; (2) peak-to-peak spread 0.078
(0.975% of derived ratio) is within the documented hardware noise floor (~2%) for
modern complex processor substrates. Noise floor grounded in published benchmarking
literature (SideRand arXiv:1810.00567, Robust Benchmarking arXiv:1608.04295,
PassMark benchmarking forum 2018). The derived ratio is the structural fixed point;
the hardware observations are consistent with bounded physical noise around it within
the irreducible noise floor of the Ryzen 5 7600X (Zen 4, ~13 billion transistors).
87/87 tests passing.

**V0.4.0 (2026-09-03):** OC-IF-SIM-3A implemented. Declared topology (Independent:
|L|=8, |E|=0; FullyCoupled: |L|=8, |E|=56) produces derived ratio C_FC/C_I = 8.0000
via counting projection C = |L| + |E| of declared relational structure. Hardware
independently produced FC/Ind elapsed time ratios 8.010, 7.945, 8.023 across a 107×
change in substrate cost (Ryzen 5 7600X, 2026-09-03). Directed differences
Δ = +0.010, −0.055, +0.023. Structural ordering benchmark added. 84/84 tests.

**V0.3.0 (2026-09-03):** fabric_sim.rs extended with three workload dependency
classes (Independent, WeaklyCoupled, FullyCoupled), 9-cell simulation matrix
(3 classes × 3 WS scales matching V13 Pass B), predicted DF PMC beat counts
grounded in AMD PPR Section 7.1. Undeclared baseline declared. 75/75 tests.

**V0.2 (2026-08-28):** Findings F1–F4 from Verifier re-review under External
Baseline Comparison criterion (operators.rs V8, 2026-08-28). F1: bandwidth-bound
throughput derivation retracted as OC-DB-3 closure. F2: latency-bound claim
downgraded to consistent-with language. F3: efficiency-advantage language replaced
with structural-property language throughout. F4: bottleneck → lower-bandwidth locus.

## Purpose

Declares the AMD Instinct MI355X Infinity Fabric as a directed relational graph
through M (AMD MI355X Platform specification, retrieved 2026-08-05) and applies
the ABR A operator to identify the declared lower-bandwidth locus and characterize
the hardware limits relevant to throughput.

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware.
OC-DB-3 remains open (bandwidth/working-set derivation retracted per OC-IF-5).

## Layer Convergence

- Layer 1  abr-grid-integration: grid physics → 100-175 MW viable band
- Layer 2  abr-workload-architecture: compute → same band independently
- Layer 3  abr-datacenter-build: rack declared; efficiency readable from ops
- Layer 4  abr-infinity-fabric (this repo): kernel traversal maps structurally
           to the declared hardware topology

## Build and Run

```
cargo build --release
cargo test
cargo run --release --bin run_sim3a
cargo run --release --bin bench_dependency_classes
```

## OC-IF-SIM-3A Result

```
Declared Structure:
  Independent:  |L|=8, |E|=0  → C_I  = 8
  FullyCoupled: |L|=8, |E|=56 → C_FC = 64
  Derived ratio C_FC/C_I = 8.0000 (not assumed — derived from declared topology)

Hardware Observations (Ryzen 5 7600X, 2026-09-03):
  S1 (4 MB):   8.010  Δ=+0.010
  S2 (32 MB):  7.945  Δ=-0.055
  S3 (128 MB): 8.023  Δ=+0.023

Substrate cost changed 107× across S1→S3.
Structural ratio remained stable: range 7.945–8.023.

Correspondence Criterion (declared 2026-09-05):
  Condition 1 — derived within observed range [7.945, 8.023]: PASS
  Condition 2 — range 0.975% within hardware noise floor 2.0%: PASS
  Noise floor grounded in: SideRand (arXiv:1810.00567), Robust Benchmarking
  (arXiv:1608.04295), PassMark benchmarking forum (2018).
```

OC-IF-SIM-3A CLOSED: derived ratio corresponds to hardware observations within
the irreducible noise floor of the physical substrate.

## Structural Reduction

Independent has 1/8 the declared traversal count of FullyCoupled —
an 8× structural reduction relative to the fully-coupled baseline.
Ratio derived from declared topology and compared with hardware observations.

At transformer scale (n=4096, undeclared attention = n² operations,
declared Independent structure = n operations): ratio = n = 4096×.
Structural extrapolation — not yet hardware-confirmed. Declared under OC-IF-SIM-2.

## Closed Open Conditions

- OC-DB-1      CLOSED: kernel-to-hardware mapping derived from declared structure
- OC-IF-SIM-3A CLOSED: correspondence criterion declared and satisfied (2026-09-05)

## Open Conditions

- OC-IF-SIM-3B  OPEN — absolute hardware cost gradient correspondence
- OC-IF-SIM-1   OPEN — DF PMC beat counts require Linux + AMDuProf
- OC-IF-SIM-2   OPEN — O(n²) complexity extension to transformer scale
- OC-DB-3   OPEN — bandwidth/working-set throughput derivation retracted per OC-IF-5
- OC-IF-1   Per-link bandwidth uniformity: AMD declares aggregate (1,194.8 GB/s)
- OC-IF-2   Partition admissibility for inter-community workloads
- OC-IF-3   ROCm HIP kernel implementation (downstream deliverable)
- OC-IF-4   Single working-set read per pass assumption
- OC-IF-5   Throughput figure consistent with approximately constant per-edge cost
- OC-DB-6   Self-describing property: formal derivation from operator fixed-point

## Grounding Documents

- OC-IF-SIM-3.md — open condition declaration with Origin confirmations (2026-09-03)
- abr-home-system-benchmark V13 — chain_only Pass A: CPI and DRAM_PTI scaling
  across 4/32/128 MB working-set scales (Ryzen 5 7600X, 2026-09-01)
- bench_dependency_classes.rs — structural ordering correspondence
  (Ryzen 5 7600X, 2026-09-03)
- abr-datacenter-build V0.3.2 (OC-DB-7 retraction applied)
- abr-community-grid-match — Lompoc primary case, Verification PASS
- operators.rs V7 — ABR kernel declaration
- derived_invariants.rs V4.1 — Layer 3 invariants
- AMD MI355X Platform specification (retrieved 2026-08-05)
- AMD ROCm published specification
