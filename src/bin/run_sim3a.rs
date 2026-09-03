// run_sim3a.rs — Metatron Dynamics, Inc.
// Runs OC-IF-SIM-3A and prints the correspondence report for Origin review.
// Bounded over D. No claim beyond D.

use abr_infinity_fabric::sim3a::{run_sim3a, format_sim3a_report};

fn main() {
    let result = run_sim3a();
    println!("{}", format_sim3a_report(&result));
}
