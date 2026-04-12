use criterion::{criterion_group, criterion_main, Criterion};
use shared::benchmarking::{
    create_criterion_benchmark, process_benchmark_results, run_dual_algorithm_benchmark, Algorithm,
    PlotConfig, TestConfig,
};
use y2024_d05::{solve_part1, solve_part1_naive};

const SEQUENCE_LENGTHS: [usize; 6] = [10, 20, 40, 80, 120, 160];
const RULE_COUNTS: [usize; 6] = [25, 50, 100, 200, 400, 600];

/// Benchmark: Sequence Length Scaling
/// Shows quadratic O(N²) vs linear O(N) scaling when varying sequence length
fn benchmark_sequence_length_scaling(c: &mut Criterion) {
    let data_dir = "data";
    let group_name = "sequence_length_scaling";

    let algorithm1 = Algorithm {
        name: "optimized",
        function: solve_part1,
    };
    let algorithm2 = Algorithm {
        name: "naive",
        function: solve_part1_naive,
    };

    let test_config = TestConfig {
        sizes: &SEQUENCE_LENGTHS,
        generate_input: generate_sequence_length_test,
    };

    run_dual_algorithm_benchmark(c, group_name, &algorithm1, &algorithm2, &test_config);

    let plot_config = PlotConfig {
        filename: "quadratic_vs_linear_sequence_scaling.svg",
        title: "Algorithmic Scaling Analysis: Execution Time vs Sequence Length",
        algorithm1_name: "Linear O(N+M) Algorithm",
        algorithm2_name: "Quadratic O(N²M) Algorithm",
        x_axis_label: "Sequence Length (N)",
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

/// Benchmark: Rule Count Scaling
/// Shows linear scaling with different slopes: O(M) vs O(N²M) when varying rule count
fn benchmark_rule_count_scaling(c: &mut Criterion) {
    let data_dir = "data";
    let group_name = "rule_count_scaling";

    let algorithm1 = Algorithm {
        name: "optimized",
        function: solve_part1,
    };
    let algorithm2 = Algorithm {
        name: "naive",
        function: solve_part1_naive,
    };

    let test_config = TestConfig {
        sizes: &RULE_COUNTS,
        generate_input: generate_rule_count_test,
    };

    run_dual_algorithm_benchmark(c, group_name, &algorithm1, &algorithm2, &test_config);

    let plot_config = PlotConfig {
        filename: "rule_count_linear_slope_comparison.svg",
        title: "Algorithmic Scaling Analysis: Execution Time vs Rule Count",
        algorithm1_name: "O(N+M) ≈ O(M) Linear",
        algorithm2_name: "O(N²M) Linear with slope N²",
        x_axis_label: "Rule Count (M)",
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

/// Test data: Variable sequence length, fixed rules
fn generate_sequence_length_test(sequence_length: usize) -> String {
    const N_RULES: usize = 80;
    const N_SEQUENCES: usize = 15;
    const PAGE_UNIVERSE: usize = 200;

    let rules = generate_fixed_rules(N_RULES, PAGE_UNIVERSE);
    let sequences = generate_length_specific_sequences(N_SEQUENCES, sequence_length, PAGE_UNIVERSE);

    format!("{}\n\n{}", rules.join("\n"), sequences.join("\n"))
}

/// Test data: Variable rule count, fixed sequences
fn generate_rule_count_test(rule_count: usize) -> String {
    const SEQUENCE_LENGTH: usize = 30;
    const N_SEQUENCES: usize = 15;
    const PAGE_UNIVERSE: usize = 150;

    let rules = generate_variable_rules(rule_count, PAGE_UNIVERSE);
    let sequences = generate_fixed_length_sequences(N_SEQUENCES, SEQUENCE_LENGTH, PAGE_UNIVERSE);

    format!("{}\n\n{}", rules.join("\n"), sequences.join("\n"))
}

fn generate_fixed_rules(n_rules: usize, page_universe: usize) -> Vec<String> {
    let mut rules = Vec::new();

    for i in 0..n_rules {
        let before = (i * 3) % (page_universe / 2);
        let after = (page_universe / 2) + ((i * 5) % (page_universe / 2));
        rules.push(format!("{before}|{after}"));
    }

    rules.sort();
    rules.dedup();
    rules
}

fn generate_variable_rules(target_rules: usize, page_universe: usize) -> Vec<String> {
    let mut rules = Vec::new();
    let max_possible = (page_universe * (page_universe - 1)) / 2;
    let actual_rules = target_rules.min(max_possible / 4);

    // Chain dependencies
    let chain_rules = actual_rules * 3 / 10;
    for i in 0..chain_rules {
        let before = i % (page_universe - 1);
        let after = before + 1;
        rules.push(format!("{before}|{after}"));
    }

    // Cross-dependencies
    let cross_rules = actual_rules * 5 / 10;
    for i in 0..cross_rules {
        let before = (i * 3) % (page_universe / 2);
        let after = (page_universe / 2) + ((i * 7) % (page_universe / 2));
        if before != after {
            rules.push(format!("{before}|{after}"));
        }
    }

    // Fan-out dependencies
    let fan_rules = actual_rules - chain_rules - cross_rules;
    for i in 0..fan_rules {
        let hub = i % (page_universe / 4);
        let target = (page_universe * 3 / 4) + (i % (page_universe / 4));
        rules.push(format!("{hub}|{target}"));
    }

    rules.sort();
    rules.dedup();
    rules.truncate(target_rules);
    rules
}

fn generate_length_specific_sequences(
    count: usize,
    target_length: usize,
    page_universe: usize,
) -> Vec<String> {
    let mut sequences = Vec::new();

    for i in 0..count {
        let length = target_length.min(page_universe);

        if i % 4 == 0 {
            let seq: Vec<String> = (0..length).map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        } else {
            let mut seq: Vec<usize> = (0..length).collect();
            for j in (0..length).step_by(4) {
                if j + 1 < length && (j / 4) % 2 == 0 {
                    seq.swap(j, j + 1);
                }
            }
            let seq: Vec<String> = seq.iter().map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        }
    }

    sequences
}

fn generate_fixed_length_sequences(
    count: usize,
    target_length: usize,
    page_universe: usize,
) -> Vec<String> {
    let mut sequences = Vec::new();

    for i in 0..count {
        let length = target_length.min(page_universe);

        if i % 3 == 0 {
            let seq: Vec<String> = (0..length).map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        } else {
            let mut seq: Vec<usize> = Vec::new();
            for j in 0..length {
                let page = match j % 4 {
                    0 => j % (page_universe / 4),
                    1 => (page_universe / 4) + (j % (page_universe / 4)),
                    2 => (page_universe / 2) + (j % (page_universe / 4)),
                    _ => (page_universe * 3 / 4) + (j % (page_universe / 4)),
                };
                seq.push(page);
            }
            let seq: Vec<String> = seq.iter().map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        }
    }

    sequences
}

criterion_group!(
    name = benches;
    config = create_criterion_benchmark("data");
    targets = benchmark_sequence_length_scaling, benchmark_rule_count_scaling
);
criterion_main!(benches);
