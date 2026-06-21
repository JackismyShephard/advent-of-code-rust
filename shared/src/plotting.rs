//! Shared plotting utilities for benchmark visualization.
//!
//! This module provides simple plotting functionality for creating
//! performance comparison charts across different days of Advent of Code.

use anyhow::{Context, Result, bail};
use itertools::Itertools;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

const CIRCLE_RADIUS: i32 = 4;
const LEGEND_LINE_LENGTH: i32 = 10;
const CHART_WIDTH: u32 = 800;
const CHART_HEIGHT: u32 = 600;

type PlotChart<'a> = ChartContext<'a, SVGBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

/// Creates a dual-algorithm performance comparison plot.
///
/// Generates an SVG chart comparing two algorithms with fixed styling:
/// - 800x600 dimensions
/// - Blue line for first algorithm, red line for second
/// - Logarithmic y-axis showing execution times
/// - Speedup factor labels
/// - Standard "Input Size (n)" / "Time (microseconds)" axes
///
/// # Parameters
/// * `filename` - Output SVG filename
/// * `title` - Chart title
/// * `algo1_name` - Name of the first algorithm
/// * `algo2_name` - Name of the second algorithm
/// * `x_axis_label` - Label for the x-axis (e.g., "Sequence Length (N)", "Rule Count (M)")
/// * `results` - Benchmark data as (input_size, time1_ns, time2_ns, speedup)
///   tuples
///
/// # Errors
///
/// Returns an error if chart creation fails.
///
/// # Examples
///
/// ```
/// # use shared::plotting::create_dual_algorithm_plot;
/// # use std::fs;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Sample benchmark data: (input_size, time1_ns, time2_ns, speedup)
/// let results = vec![
///     (1000, 50000.0, 25000.0, 2.0), // 1000 elements: 50μs vs 25μs,
///                                      // 2x speedup
///     (5000, 250000.0, 100000.0, 2.5), // 5000 elements: 250μs vs 100μs,
///                                       // 2.5x speedup
/// ];
///
/// create_dual_algorithm_plot(
///     "benchmark_comparison.svg",
///     "Algorithm Performance Comparison",
///     "Naive Algorithm",
///     "Optimized Algorithm",
///     "Input Size (N)",
///     &results
/// )?;
/// # fs::remove_file("benchmark_comparison.svg").ok();
/// # Ok(())
/// # }
/// ```
pub fn create_dual_algorithm_plot(
    filename: &str,
    title: &str,
    algo1_name: &str,
    algo2_name: &str,
    x_axis_label: &str,
    results: &[(usize, f64, f64, f64)],
) -> Result<()> {
    let (root, mut chart) = setup_dual_performance_chart(filename, title, results)?;

    // Configure mesh for performance benchmark charts
    chart
        .configure_mesh()
        .x_desc(x_axis_label)
        .y_desc("Time (microseconds)")
        .x_label_formatter(&|x| format!("{x:.0}"))
        .y_label_formatter(&|y| format!("{:.0}", 10f64.powf(*y) / 1000.0))
        .draw()?;

    // Plot both algorithms
    plot_performance_line(&mut chart, results, 0, &BLUE, algo1_name)?;
    plot_performance_line(&mut chart, results, 1, &RED, algo2_name)?;

    // Add speedup labels above the second algorithm line
    add_speedup_labels(&mut chart, results)?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("✅ Performance plot saved as '{filename}'");
    Ok(())
}

/// Sets up the chart layout and coordinate system for dual-algorithm
/// performance benchmarks.
///
/// Creates the SVG backend, determines appropriate axis ranges from timing
/// data, and builds the chart with logarithmic y-axis scaling for performance
/// visualization.
///
/// # Parameters
/// * `filename` - Output SVG filename
/// * `title` - Chart title
/// * `results` - Benchmark data used to determine axis ranges
///
/// # Returns
/// drawing_area and configured chart, ready for mesh configuration and data
/// plotting
///
/// # Errors
///
/// Returns an error if the chart setup fails (e.g., invalid ranges, SVG
/// backend issues).
fn setup_dual_performance_chart<'a>(
    filename: &'a str,
    title: &'a str,
    results: &[(usize, f64, f64, f64)],
) -> Result<(
    DrawingArea<SVGBackend<'a>, plotters::coord::Shift>,
    PlotChart<'a>,
)> {
    let root = SVGBackend::new(filename, (CHART_WIDTH, CHART_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = results
        .iter()
        .map(|(size, _, _, _)| *size)
        .max()
        .context("No data points to plot")?;
    let times: Vec<f64> = results
        .iter()
        .flat_map(|(_, t1, t2, _)| [*t1, *t2])
        .collect();
    let (min_time, max_time) = (
        times.iter().copied().fold(f64::INFINITY, f64::min),
        times.iter().copied().fold(0.0, f64::max),
    );

    let chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
        .margin(50)
        .margin_top(50)
        .margin_bottom(40)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0f64..(max_size as f64 * 1.1),
            (min_time * 0.5).log10()..(max_time * 2.0).log10(),
        )?;

    Ok((root, chart))
}

/// Plots a single algorithm's performance line.
///
/// Extracts timing data for one algorithm, applies logarithmic transformation,
/// and draws a line with points for that algorithm's performance.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing operations
/// * `results` - Benchmark data as tuples of (input_size, time1_ns, time2_ns,
///   speedup)
/// * `time_index` - Which time column to use (0 for first algorithm, 1 for
///   second)
/// * `color` - Color for the line and markers
/// * `label` - Label for the legend entry
/// # Errors
///
/// Returns an error if chart drawing operations fail (SVG backend errors,
/// invalid coordinates).
///
fn plot_performance_line<'a>(
    chart: &mut PlotChart<'a>,
    results: &[(usize, f64, f64, f64)],
    time_index: usize,
    color: &'a RGBColor,
    label: &str,
) -> Result<()> {
    let points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, time1, time2, time3)| {
            let time = match time_index {
                0 => *time1,
                1 => *time2,
                2 => *time3,
                _ => bail!("Invalid time_index: {time_index}. Must be 0, 1, or 2"),
            };
            Ok((*size as f64, time.log10()))
        })
        .try_collect()?;

    draw_line_with_points(chart, &points, color, label)
}

/// Draws a performance line with circular markers and legend entry.
///
/// Helper function that creates both the line series and point markers
/// for a single algorithm's performance data.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing operations
/// * `points` - Array of (x, y) coordinates representing algorithm
///   performance data
/// * `color` - Reference to the RGB color for drawing the line and markers
/// * `label` - Text label for the legend entry describing this algorithm
///
/// # Errors
///
/// Returns an error if drawing fails.
fn draw_line_with_points<'a>(
    chart: &mut PlotChart<'a>,
    points: &[(f64, f64)],
    color: &'a RGBColor,
    label: &str,
) -> Result<()> {
    chart
        .draw_series(LineSeries::new(points.iter().copied(), color))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + LEGEND_LINE_LENGTH, y)], *color));
    chart.draw_series(
        points
            .iter()
            .map(|&(x, y)| Circle::new((x, y), CIRCLE_RADIUS, color.filled())),
    )?;
    Ok(())
}

/// Adds speedup factor labels above the second algorithm performance line.
///
/// Places text annotations showing the performance improvement factor
/// at each data point for easy interpretation of results.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing text labels
/// * `results` - Benchmark data as tuples of (input_size, time1_ns, time2_ns,
///   speedup_factor)
///
/// # Errors
///
/// Returns an error if drawing the labels fails (e.g., invalid coordinates,
/// SVG backend issues).
fn add_speedup_labels(chart: &mut PlotChart<'_>, results: &[(usize, f64, f64, f64)]) -> Result<()> {
    let labels: Vec<_> = results
        .iter()
        .map(|(size, _, time2, speedup)| {
            Text::new(
                format!("{speedup:.1}x"),
                (*size as f64, time2.log10() * 1.05),
                ("sans-serif", 12),
            )
        })
        .collect();

    chart.draw_series(labels)?;
    Ok(())
}
