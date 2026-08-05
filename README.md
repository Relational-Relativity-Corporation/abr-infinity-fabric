# abr-infinity-fabric

**ABR Infinity Fabric — AMD Instinct MI355X topology declared as relational structure.**

Closes OC-DB-1: formal mapping of ABR operator traversal to Infinity Fabric hardware. 57/57 tests. Four-layer convergence confirmed.

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

---

## What This Repository Establishes

This repository formally closes **OC-DB-1** from `abr-datacenter-build`:

> *The ABR operator traversal pattern on sparse declared graphs is not yet formally mapped to AMD Infinity Fabric topology. The efficiency advantage is structurally derived; the hardware-specific throughput is an open condition.*

**Closure argument:**

1. The Infinity Fabric is declared as a directed relational graph through M — AMD MI355X Platform specification, retrieved 2026-08-05.
2. The ABR A operator is applied to the fabric bandwidth field, sign consistent with the kernel declaration (`operators.rs` V8).
3. The fabric switch (1,194.8 GB/s aggregate) is identified as the bandwidth bottleneck against module bandwidth (8,000 GB/s per module).
4. Community analysis working sets (1 MB generous upper bound) fit entirely within module HBM3E (288 GB) — resident without eviction.
5. Independent community analyses require **zero inter-module communication** — the fabric bottleneck is not active for this workload class.
6. The ABR kernel executes at full declared HBM3E bandwidth (8.0 TB/s per module) for community analysis workloads.
7. Throughput derived structurally: ~7.6M analyses/second per module, ~61M analyses/second at rack scale (8 modules).

**OC-DB-3** (community queue throughput) is closed structurally as a consequence. Correspondence requires instrument measurement — see open conditions.

---

## Four-Layer Convergence

This repository is Layer 4 in a four-layer independently derived argument:

| Layer | Repository | Finding |
|---|---|---|
| 1 | `abr-grid-integration` | Grid physics → 100–175 MW viable band |
| 2 | `abr-workload-architecture` | Compute architecture → same band independently |
| 3 | `abr-datacenter-build` | Rack declared as relational structure; efficiency readable from operator output |
| 4 | `abr-infinity-fabric` (this repo) | ABR kernel maps to MI355X hardware natively at full HBM3E bandwidth |

Four independent derivations converging on the same declared structure.

---

## Declared Provenance Chain

Every quantity in this repository traces to a declared observable through M:

| Quantity | Source | Provenance |
|---|---|---|
| Fabric aggregate bandwidth (1,194.8 GB/s) | AMD MI355X Platform specification | Published manufacturer specification through M |
| Module memory bandwidth (8.0 TB/s) | AMD MI355X Platform specification | Published manufacturer specification through M |
| Module HBM3E capacity (288 GB) | AMD MI355X Platform specification | Published manufacturer specification through M |
| Community working set (1 MB) | `abr-community-grid-match` declared structure | Declared observable structure through M |
| Reference LLM workload (140 GB) | Meta LLaMA 3 70B model card | Published model specification through M |
| Software stack (ROCm 7.0 / HIP) | AMD ROCm published specification | Published software specification through M |

---

## Module Structure

```
src/
├── fabric_topology.rs       — Infinity Fabric as declared directed graph (9 loci, 16 edges)
├── fabric_field.rs          — Bandwidth field; A operator; bottleneck identification
├── workload_graph.rs        — Community analysis graph as partitionable workload
├── partition_mapping.rs     — Derives partition; checks AC-1 through AC-4; closes OC-DB-1
├── kernel_execution.rs      — Execution model; ROCm HIP interface declaration
├── throughput_invariants.rs — Structural throughput derivation; closes OC-DB-3
└── convergence.rs           — Full chain integration test; formal OC-DB-1 closure
```

---

## Open Conditions

| ID | Description | Status |
|---|---|---|
| OC-IF-1 | Per-link bandwidth uniformity: AMD declares aggregate (1,194.8 GB/s); per-link derived assuming uniform distribution | Open |
| OC-IF-2 | Partition admissibility for inter-community workloads: closure holds for independent analyses; workloads with inter-community dependencies require separate derivation | Open |
| OC-IF-3 | ROCm HIP kernel implementation: execution model declared; kernel source is a downstream deliverable | Open |
| OC-IF-4 | Single working-set read per ABR pass: declared as reference execution model for sparse resident graphs; formal derivation from operator traversal count not yet complete | Open |
| OC-DB-6 | Self-describing property: supported by declared execution model; formal derivation from operator fixed-point not yet complete | Open |

---

## Running Tests

```bash
cargo test
```

Expected output: **57 passed; 0 failed**

---

## Conformance Statement

A convergence PASS is a conformance statement — not a correspondence claim. Conformance confirms that the declared structure is internally consistent and that every quantity traces to a declared observable through M. Correspondence — that the declared throughput matches measured hardware throughput — requires instrument measurement (OC-IF-3, OC-IF-4).

---

## Grounding Documents

- `abr-datacenter-build` V0.2 — open conditions OC-DB-1 through OC-DB-6
- `abr-community-grid-match` — Lompoc primary case, Verification PASS
- `operators.rs` V8 — ABR kernel declaration
- `derived_invariants.rs` V4.1 — Layer 3 invariants
- AMD MI355X Platform specification — https://www.amd.com/en/products/accelerators/instinct/mi350/mi355x/platform.html (retrieved 2026-08-05)
- AMD ROCm — https://www.amd.com/en/products/software/rocm.html

---

## License

Apache License 2.0 — see LICENSE file.

*Metatron Dynamics, Inc. Robin Macomber. Bounded over D. No claim beyond D.*
