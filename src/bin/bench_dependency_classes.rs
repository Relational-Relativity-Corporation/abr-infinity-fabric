// bench_dependency_classes.rs — Metatron Dynamics, Inc.
// Structural ordering correspondence measurement.
// Bounded over D. No claim beyond D.
//
// ── Purpose ──────────────────────────────────────────────────────────────────
//
// Validates that hardware execution time preserves the simulation's predicted
// ordering (Independent < WeaklyCoupled < FullyCoupled) when each dependency
// class is instantiated with its naturally implied number of memory relations
// as declared in fabric_sim.rs.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Measured observable: elapsed wall-clock execution time (ns).
//   Instrument: std::time::Instant (Windows high-resolution timer).
//   Each workload class run N_WARMUP passes before N_MEASURE passes.
//   Reported: mean elapsed time over N_MEASURE passes.
//
// Derived projection: elapsed_ns / N_REGIONS.
//   Preserves: normalized cost per declared region.
//   Discards: total workload scale.
//
// Validation criterion (not an observable):
//   Independent < WeaklyCoupled < FullyCoupled at DRAM-resident scale.
//
// ── Operation Count — Part of Declared Structural Difference ─────────────────
//
// Operation count is part of the declared structural difference.
// This experiment does not isolate coupling cost from work count.
// The finding is: hardware preserves simulation ordering when each class
// is instantiated with its naturally implied number of memory relations.
//
//   Independent:    n_regions reads per pass (1 buffer per region)
//   WeaklyCoupled:  n_regions × (1 + N_WEAK_DEPS) reads per pass
//   FullyCoupled:   n_regions × n_regions reads per pass
//
// ── Working-Set Scale Hardware Grounding ─────────────────────────────────────
//
// Ryzen 5 7600X declared cache hierarchy (AMD published specification):
//   L2: 1 MB per core, 6 MB total
//   L3: 32 MB shared
//
// Declared scales:
//   L2-resident  (4 MB total):   fits within 6 MB L2
//   L3-resident  (32 MB total):  at L3 boundary
//   DRAM-resident (128 MB total): exceeds 32 MB L3
//
// OC-BENCH-1 OPEN: DRAM-resident label requires hardware confirmation via
//   timing discontinuity between L3 and DRAM scales. If no discontinuity
//   is observed, "DRAM-resident" label is inadmissible.
//
// ── Pointer-Chase Pattern ─────────────────────────────────────────────────────
//
// Reads are structured as a pointer-chase through each buffer to defeat
// hardware prefetch and ensure cache pressure is genuine at DRAM scale.
// Each buffer is initialized with a pseudo-random permutation so that
// successive reads follow unpredictable offsets — no stride prefetch.
//
// ── Scope ────────────────────────────────────────────────────────────────────
//
// This measurement produces timing observables on Ryzen 5 7600X.
// It does not produce DF PMC beat counts (OC-IF-SIM-1 OPEN).
// Correspondence to fabric_sim.rs predicted beat counts requires
// DF PMC measurement via AMDuProf on Linux (declared future work).

use std::time::Instant;

// ── Constants ─────────────────────────────────────────────────────────────────

/// Number of declared data regions. Matches fabric_sim.rs WorkloadParams.
const N_REGIONS: usize = 8;

/// Weak dependency count per region. Matches fabric_sim.rs n_weak_deps_per_region.
const N_WEAK_DEPS: usize = 2;

/// Warmup passes before measurement (not recorded).
const N_WARMUP: usize = 3;

/// Measurement passes (mean taken over these).
const N_MEASURE: usize = 10;

/// Working-set sizes. Match fabric_sim.rs declared scales.
/// L2-resident:   8 × 512 KB  =   4 MB (fits within 6 MB L2)
/// L3-resident:   8 × 4 MB    =  32 MB (at L3 boundary)
/// DRAM-resident: 8 × 16 MB   = 128 MB (exceeds 32 MB L3)
const SCALES: &[(&str, usize)] = &[
    ("L2-resident  ( 4 MB total, 512 KB/region)", 512 * 1024),
    ("L3-resident  (32 MB total,   4 MB/region)", 4 * 1024 * 1024),
    ("DRAM-resident(128 MB total, 16 MB/region)", 16 * 1024 * 1024),
];

// ── Buffer Initialization ─────────────────────────────────────────────────────

/// Initialize buffer as a pointer-chase permutation.
/// Each element contains the index of the next element to read.
/// Uses a linear congruential shuffle to defeat prefetch.
fn init_chase_buffer(buf: &mut Vec<usize>) {
    let n = buf.len();
    // Initialize as identity permutation.
    for i in 0..n {
        buf[i] = i;
    }
    // Fisher-Yates shuffle with LCG random — deterministic, unpredictable stride.
    let mut rng: u64 = 0xdeadbeef_cafebabe;
    for i in (1..n).rev() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let j = (rng >> 33) as usize % (i + 1);
        buf.swap(i, j);
    }
}

/// Allocate and initialize n_regions buffers of size_bytes each.
/// Returns Vec of chase buffers (each buffer is Vec<usize>).
fn allocate_buffers(n_regions: usize, size_bytes: usize) -> Vec<Vec<usize>> {
    let n_elements = size_bytes / std::mem::size_of::<usize>();
    let mut buffers = Vec::with_capacity(n_regions);
    for _ in 0..n_regions {
        let mut buf = vec![0usize; n_elements];
        init_chase_buffer(&mut buf);
        buffers.push(buf);
    }
    buffers
}

// ── Workload Implementations ──────────────────────────────────────────────────

/// Independent class: each region reads only its own buffer.
/// Operations per pass: n_regions × n_elements pointer chases.
#[inline(never)]
fn run_independent(buffers: &[Vec<usize>], n_chases: usize) -> usize {
    let mut sink = 0usize;
    for buf in buffers {
        let mut idx = 0usize;
        for _ in 0..n_chases {
            idx = buf[idx % buf.len()];
        }
        sink ^= idx;
    }
    sink
}

/// WeaklyCoupled class: each region reads its own buffer plus N_WEAK_DEPS
/// adjacent regions' buffers (modular wrap). Matches fabric_sim.rs
/// WeaklyCoupled dependency graph declaration.
/// Operations per pass: n_regions × (1 + N_WEAK_DEPS) × n_elements chases.
#[inline(never)]
fn run_weakly_coupled(buffers: &[Vec<usize>], n_chases: usize) -> usize {
    let n = buffers.len();
    let mut sink = 0usize;
    for i in 0..n {
        // Own buffer.
        let mut idx = 0usize;
        for _ in 0..n_chases {
            idx = buffers[i][idx % buffers[i].len()];
        }
        sink ^= idx;
        // Declared dependency buffers (adjacent regions, modular).
        for k in 1..=N_WEAK_DEPS {
            let src = (i + k) % n;
            let mut idx2 = 0usize;
            for _ in 0..n_chases {
                idx2 = buffers[src][idx2 % buffers[src].len()];
            }
            sink ^= idx2;
        }
    }
    sink
}

/// FullyCoupled class: each region reads its own buffer plus all other
/// regions' buffers. Matches fabric_sim.rs FullyCoupled dependency graph.
/// Operations per pass: n_regions × n_regions × n_elements chases.
#[inline(never)]
fn run_fully_coupled(buffers: &[Vec<usize>], n_chases: usize) -> usize {
    let n = buffers.len();
    let mut sink = 0usize;
    for i in 0..n {
        for j in 0..n {
            let mut idx = 0usize;
            for _ in 0..n_chases {
                idx = buffers[j][idx % buffers[j].len()];
            }
            sink ^= idx;
        }
    }
    sink
}

// ── Measurement ───────────────────────────────────────────────────────────────

struct MeasurementResult {
    class_label: &'static str,
    scale_label: &'static str,
    mean_ns: f64,
    mean_ns_per_region: f64,
    n_operations: usize,
}

fn measure_class(
    class_label: &'static str,
    scale_label: &'static str,
    buffers: &[Vec<usize>],
    n_chases: usize,
    run_fn: impl Fn(&[Vec<usize>], usize) -> usize,
) -> MeasurementResult {
    // Warmup — not recorded.
    let mut sink = 0usize;
    for _ in 0..N_WARMUP {
        sink ^= run_fn(buffers, n_chases);
    }

    // Measurement passes.
    let mut total_ns = 0u64;
    for _ in 0..N_MEASURE {
        let t0 = Instant::now();
        sink ^= run_fn(buffers, n_chases);
        total_ns += t0.elapsed().as_nanos() as u64;
    }

    // Prevent dead-code elimination.
    if sink == usize::MAX { println!("sink={}", sink); }

    let mean_ns = total_ns as f64 / N_MEASURE as f64;
    let mean_ns_per_region = mean_ns / N_REGIONS as f64;

    // Operation count: n_chases per buffer read, total buffer reads per pass.
    let n_buf_reads = match class_label {
        "Independent"    => N_REGIONS,
        "WeaklyCoupled"  => N_REGIONS * (1 + N_WEAK_DEPS),
        "FullyCoupled"   => N_REGIONS * N_REGIONS,
        _                => 0,
    };
    let n_operations = n_buf_reads * n_chases;

    MeasurementResult {
        class_label,
        scale_label,
        mean_ns,
        mean_ns_per_region,
        n_operations,
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    println!("ABR Infinity Fabric — Structural Ordering Correspondence Measurement");
    println!("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.");
    println!();
    println!("Declared observable: elapsed wall-clock time (ns) via std::time::Instant.");
    println!("Derived projection:  elapsed_ns / N_REGIONS (normalized per region).");
    println!("Validation criterion: Independent < WeaklyCoupled < FullyCoupled at DRAM scale.");
    println!("Operation count is part of declared structural difference (Option B).");
    println!("OC-IF-SIM-1 OPEN: DF PMC beat counts require Linux + AMDuProf.");
    println!("OC-BENCH-1 OPEN: DRAM label requires timing discontinuity confirmation.");
    println!();
    println!("N_REGIONS={} | N_WEAK_DEPS={} | N_WARMUP={} | N_MEASURE={}",
        N_REGIONS, N_WEAK_DEPS, N_WARMUP, N_MEASURE);
    println!();

    // Chase depth: enough pointer chases per buffer read to ensure
    // the full buffer is traversed. Use n_elements as chase depth
    // so every cache line is touched at least once.
    // Declared: n_chases = buffer_size_bytes / sizeof(usize).

    let mut all_results: Vec<MeasurementResult> = Vec::new();

    for &(scale_label, size_bytes) in SCALES {
        println!("── Scale: {} ────────────────────────────────", scale_label);
        println!("  Allocating {} buffers × {} bytes...", N_REGIONS, size_bytes);

        let buffers = allocate_buffers(N_REGIONS, size_bytes);
        let n_elements = size_bytes / std::mem::size_of::<usize>();
        let n_chases = n_elements; // traverse full buffer per read

        println!("  n_elements={} | n_chases={}", n_elements, n_chases);
        println!();

        let ind = measure_class("Independent",   scale_label, &buffers, n_chases, run_independent);
        let wk  = measure_class("WeaklyCoupled", scale_label, &buffers, n_chases, run_weakly_coupled);
        let fc  = measure_class("FullyCoupled",  scale_label, &buffers, n_chases, run_fully_coupled);

        // Print scale results.
        println!("  {:<16} | mean={:>12.0} ns | ns/region={:>10.1} | ops={:>12}",
            ind.class_label, ind.mean_ns, ind.mean_ns_per_region, ind.n_operations);
        println!("  {:<16} | mean={:>12.0} ns | ns/region={:>10.1} | ops={:>12}",
            wk.class_label,  wk.mean_ns,  wk.mean_ns_per_region,  wk.n_operations);
        println!("  {:<16} | mean={:>12.0} ns | ns/region={:>10.1} | ops={:>12}",
            fc.class_label,  fc.mean_ns,  fc.mean_ns_per_region,  fc.n_operations);

        // Ordering check.
        let order_holds = ind.mean_ns < wk.mean_ns && wk.mean_ns < fc.mean_ns;
        println!();
        println!("  Ordering check (Ind < WK < FC): {}", if order_holds { "PASS" } else { "FAIL" });
        println!();

        all_results.push(ind);
        all_results.push(wk);
        all_results.push(fc);
    }

    // ── Summary Table ─────────────────────────────────────────────────────────
    println!("── Summary Table ────────────────────────────────────────────────────────");
    println!("{:<16} {:<36} {:>14} {:>14} {:>12}",
        "Class", "Scale", "Mean (ns)", "ns/region", "Ops");
    println!("{}", "-".repeat(96));
    for r in &all_results {
        println!("{:<16} {:<36} {:>14.0} {:>14.1} {:>12}",
            r.class_label, r.scale_label, r.mean_ns, r.mean_ns_per_region, r.n_operations);
    }

    // ── OC-BENCH-1: Discontinuity Check ──────────────────────────────────────
    println!();
    println!("── OC-BENCH-1: Scale Discontinuity Check ───────────────────────────────");
    println!("DRAM-resident label requires timing discontinuity vs L3-resident scale.");
    println!("Checking Independent class (cleanest signal — no dependency overhead):");

    let ind_l2   = all_results.iter().find(|r| r.class_label == "Independent" && r.scale_label.contains("4 MB")).unwrap();
    let ind_l3   = all_results.iter().find(|r| r.class_label == "Independent" && r.scale_label.contains("32 MB")).unwrap();
    let ind_dram = all_results.iter().find(|r| r.class_label == "Independent" && r.scale_label.contains("128 MB")).unwrap();

    let l2_to_l3_ratio   = ind_l3.mean_ns   / ind_l2.mean_ns;
    let l3_to_dram_ratio = ind_dram.mean_ns  / ind_l3.mean_ns;

    println!("  L2→L3   ratio: {:.2}x", l2_to_l3_ratio);
    println!("  L3→DRAM ratio: {:.2}x", l3_to_dram_ratio);

    // A DRAM discontinuity should produce a significantly larger ratio
    // than the L2→L3 transition, due to DRAM latency (~60-80 ns) vs
    // L3 latency (~30-40 ns cycles on Zen 4).
    let discontinuity_observed = l3_to_dram_ratio > l2_to_l3_ratio * 1.5;
    println!("  Discontinuity observed (L3→DRAM ratio > 1.5× L2→L3 ratio): {}",
        if discontinuity_observed { "YES — DRAM-resident label supported" }
        else { "NO — OC-BENCH-1 remains open; DRAM label inadmissible" });

    // ── Validation Criterion ──────────────────────────────────────────────────
    println!();
    println!("── Validation Criterion ─────────────────────────────────────────────────");
    let dram_ind  = all_results.iter().find(|r| r.class_label == "Independent"   && r.scale_label.contains("128 MB")).unwrap();
    let dram_wk   = all_results.iter().find(|r| r.class_label == "WeaklyCoupled" && r.scale_label.contains("128 MB")).unwrap();
    let dram_fc   = all_results.iter().find(|r| r.class_label == "FullyCoupled"  && r.scale_label.contains("128 MB")).unwrap();

    let ordering_holds = dram_ind.mean_ns < dram_wk.mean_ns && dram_wk.mean_ns < dram_fc.mean_ns;
    println!("Independent < WeaklyCoupled < FullyCoupled at DRAM-resident scale: {}",
        if ordering_holds { "PASS" } else { "FAIL" });

    if ordering_holds {
        println!();
        println!("FINDING: Hardware preserves simulation ordering when each dependency class");
        println!("is instantiated with its naturally implied number of memory relations.");
        println!("Operation count is part of the structural difference (Option B, declared).");
        println!("OC-IF-SIM-1 remains OPEN: DF PMC beat counts require Linux + AMDuProf.");
    } else {
        println!();
        println!("FINDING: Ordering not preserved. Review workload implementation.");
        println!("Possible cause: cache effects dominate at this working-set scale.");
    }
}
