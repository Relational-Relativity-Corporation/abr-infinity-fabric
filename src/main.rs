// main.rs — Metatron Dynamics, Inc.
// Runs the fabric_sim simulation matrix and prints the predicted
// DF PMC beat count comparison table for Origin review before push.
// Bounded over D. No claim beyond D.

use abr_infinity_fabric::fabric_sim::{run_simulation_matrix, format_comparison_table};

fn main() {
    let results = run_simulation_matrix();
    println!("{}", format_comparison_table(&results));

    // Print raw numbers for each cell so Origin can inspect directly.
    println!("── Raw Cell Values ─────────────────────────────────────────────");
    for r in &results {
        let class_str = match r.class {
            abr_infinity_fabric::fabric_sim::WorkloadClass::Independent   => "Independent",
            abr_infinity_fabric::fabric_sim::WorkloadClass::WeaklyCoupled => "WeaklyCoupled",
            abr_infinity_fabric::fabric_sim::WorkloadClass::FullyCoupled  => "FullyCoupled",
        };
        println!(
            "{} | {} | dep_edges={} | cs_ws={} | cs_dep={} | cs_total={} | xgmi={} | if_active={}",
            r.scale_label,
            class_str,
            r.n_dependency_edges,
            r.predicted_cs_read_beats_working_set,
            r.predicted_cs_read_beats_dependencies,
            r.predicted_cs_read_beats_total,
            r.predicted_xgmi_beats,
            r.activates_lower_bandwidth_locus,
        );
    }
}
