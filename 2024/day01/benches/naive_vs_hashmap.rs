use criterion::{criterion_group, criterion_main, Criterion};
use shared::benchmarking::{
    create_criterion_benchmark, process_benchmark_results, run_dual_algorithm_benchmark, Algorithm,
    PlotConfig, TestConfig,
};
use y2024_d01::{solve_part2, solve_part2_naive};

const SIZES: [usize; 6] = [500, 1000, 2000, 5000, 8000, 12000];

/// Criterion benchmark with JSON extraction and co-located output
fn benchmark_algorithms(c: &mut Criterion) {
    let data_dir = "data";
    let group_name = "criterion";

    // Algorithm definitions
    let algorithm1 = Algorithm {
        name: "hashmap",
        function: solve_part2 as fn(&str) -> _,
    };
    let algorithm2 = Algorithm {
        name: "naive",
        function: solve_part2_naive as fn(&str) -> _,
    };

    // Test configuration
    let test_config = TestConfig {
        sizes: &SIZES,
        generate_input: generate_test_input,
    };

    // Run the benchmark
    run_dual_algorithm_benchmark(c, group_name, &algorithm1, &algorithm2, &test_config);

    // Process results and generate outputs
    let plot_config = PlotConfig {
        filename: "hashmap_vs_naive.svg",
        title: "Day 1: HashMap vs Naive Algorithm Performance",
        algorithm1_name: "O(n) HashMap Solution",
        algorithm2_name: "O(n²) Naive Algorithm",
        x_axis_label: "Number of Sequences (n)",
    };

    process_benchmark_results(
        data_dir,
        group_name,
        &algorithm1,
        &algorithm2,
        &plot_config,
        &test_config,
    );
}

/// Generates synthetic test input for performance benchmarking.
///
/// Creates deterministic pairs of numbers in the format required by Day 1:
/// two space-separated integers per line. Uses modular arithmetic to ensure
/// controlled distribution and repeatable benchmarks across runs.
///
/// The left column cycles through 1-9999, while the right column uses a
/// different cycle (multiplied by 7) to create realistic similarity patterns
/// without perfect correlation.
///
/// # Parameters
/// * `size` - Number of number pairs to generate (lines of output)
///
/// # Returns
/// String representation of number pairs in Day 1 input format:
/// ```text
/// 1 1
/// 2 8
/// 3 15
/// ...
/// ```
///
/// # Examples
/// ```
/// let input = generate_test_input(3);
/// assert_eq!(input, "1 1\n2 8\n3 15");
/// ```
fn generate_test_input(size: usize) -> String {
    (0..size)
        .map(|i| format!("{} {}", (i % 9999) + 1, ((i * 7) % 9999) + 1))
        .collect::<Vec<String>>()
        .join("\n")
}

criterion_group!(
    name = benches;
    config = create_criterion_benchmark("data");
    targets = benchmark_algorithms
);
criterion_main!(benches);
