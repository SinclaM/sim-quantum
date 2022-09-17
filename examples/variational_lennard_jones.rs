use sim_quantum::physics::lennard_jones_potential;
use sim_quantum::physics::variational::VariationalSolver;

use std::fs;

use plotters::prelude::*;

fn main() {
    // Solve the time-independent schrodinger equation using the variational Monte-Carlo method.
    const STEP_SIZE: f64 = 0.001;
    const MIN_X: f64 = 0.5;
    const MAX_X: f64 = 5.0;

    let mut solver = VariationalSolver::new(
        STEP_SIZE,
        lennard_jones_potential,
        MIN_X,
        MAX_X,
    );

    // Plot the data
    fs::create_dir_all("img").expect("Failed to create image directory");
    let root_area = BitMapBackend::new("img/variational_lennard_jones.png", (1280, 720))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            "Wavefunction in a Lennard-Jones potential using the variational Monte-Carlo method",
            ("sans-serif", 40),
        )
        .build_cartesian_2d(solver.x_min..solver.x_max, -0.0..2.0)
        .unwrap();

    ctx.configure_mesh()
        .x_desc("x")
        .y_desc("ψ")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    ctx.draw_series(solver.wavefunction_points().iter().map(|point| {
        Circle::new(
            *point,
            2,
            plotters::style::ShapeStyle {
                color: BLUE.mix(1.0),
                filled: true,
                stroke_width: 1,
            },
        )
    }))
    .unwrap()
    .label(format!("E = {:.3}", solver.current_energy()))
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .label_font(("sans-serif", 20))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}