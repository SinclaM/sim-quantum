use quantum_mechanics::physics::harmonic_potential;
use quantum_mechanics::physics::matching::MatchingSolver;

use std::fs;

use plotters::prelude::*;

fn main() {
    // Solve the time-independent schrodinger equation using the shooting method for
    // odd parity wavefunctions.
    const STEP_SIZE: f64 = 0.0001;
    const INITIAL_ENERGY: f64 = 0.01;
    const INITIAL_ENERGY_STEP_SIZE: f64 = 0.1;
    const ENERGY_STEP_SIZE_CUTOFF: f64 = 0.001;
    const MIN_X: f64 = -2.5;
    const MAX_X: f64 = 2.5;
    const MATCH_X_VAL: f64 = MIN_X + (MAX_X - MIN_X) / 3.0;
    let match_idx = ((MATCH_X_VAL - MIN_X) / STEP_SIZE) as usize;
    let mut solver = MatchingSolver::new(
        STEP_SIZE,
        INITIAL_ENERGY,
        INITIAL_ENERGY_STEP_SIZE,
        harmonic_potential,
        ENERGY_STEP_SIZE_CUTOFF,
        MIN_X,
        MAX_X,
        match_idx,
    )
    .unwrap();
    solver.solve();

    // Write the output to a data file
    fs::create_dir_all("img").expect("Failed to create image directory");
    let root_area = BitMapBackend::new("img/harmonic_oscillator_matching_method.png", (1280, 720))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            "Harmonic oscillator wavefunction using the matching method",
            ("sans-serif", 40),
        )
        .build_cartesian_2d(solver.x_min..solver.x_max, -0.0..1e-3_f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(solver.wavefunction_points().iter().map(|point| {
        Circle::new(
            *point,
            2,
            plotters::style::ShapeStyle {
                color: if point.0 <= MATCH_X_VAL {
                    BLUE.mix(1.0)
                } else {
                    RED.mix(1.0)
                },
                filled: true,
                stroke_width: 1,
            },
        )
    }))
    .unwrap()
    .label(format!("E = {:.3}", solver.energy))
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .label_font(("sans-serif", 20))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
