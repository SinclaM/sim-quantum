//! Shooting method for solving the time-independent Schrodinger equation
//! in one dimension. The shooting method is only applicable for even
//! potentials (symmetric about x = 0).

use crate::physics::solvers::Solver;

/// Configuration for the shooting solver
#[derive(Clone)]
pub struct ShootingConfig {
    pub x_max: f64,
    pub step_size: f64,
    pub initial_energy: f64,
    pub intitial_energy_step_size: f64,
    pub wavefunction_cutoff: f64,
    pub potential: fn(f64) -> f64,
    pub energy_step_size_cutoff: f64,
    pub parity: Parity,
}

/// A solver that looks for solutions of the desired parity
/// using the shooting method.
pub struct ShootingSolver {
    pub config: ShootingConfig,
    energy: f64,
    energy_step_size: f64,
    wavefunction: Vec<f64>,
    last_diverge: f64,
}

impl ShootingSolver {
    /// Applies the finite difference approximation to find value of wavefunction
    /// one position forward, using the two most recent values.
    fn step(&mut self) {
        let i = self.wavefunction.len() - 1;
        self.wavefunction.push(
            2.0 * self.wavefunction[i]
                - self.wavefunction[i - 1]
                - 2.0
                    * (self.energy - (self.config.potential)((i as f64) * self.config.step_size))
                    * (self.config.step_size * self.config.step_size)
                    * self.wavefunction[i],
        );
    }

    /// Determines if the wavefunction is diverging to infinity (positive or negative).
    fn is_diverging(&self) -> bool {
        self.wavefunction.last().unwrap().abs() > self.config.wavefunction_cutoff
    }

    /// Approximates the wavefunction for the current energy. Stops when it has
    /// computed the requested number of steps, or if the wavefunction begins
    /// diverging.
    fn compute_wavefunction(&mut self) {
        self.reset_wavefunction();

        let steps = (self.config.x_max / self.config.step_size).round() as usize + 1;
        for _ in 2..=steps {
            if self.is_diverging() {
                break;
            }
            self.step();
        }
    }

    /// Resets the wavefunction vector.
    fn reset_wavefunction(&mut self) {
        self.wavefunction.clear();
        match self.config.parity {
            Parity::Even => {
                self.wavefunction.push(1.0);
                self.wavefunction.push(1.0);
            }
            Parity::Odd => {
                self.wavefunction.push(0.0);
                self.wavefunction.push(self.config.step_size);
            }
        }
    }
}

impl Solver for ShootingSolver {
    type CONFIG = ShootingConfig;
    
    fn new(config: &Self::CONFIG) -> Self {
        let steps = (config.x_max / config.step_size).round() as usize + 1;
        let wavefunction: Vec<f64> = Vec::with_capacity(steps);

        ShootingSolver {
            config: config.clone(),
            energy: config.initial_energy,
            energy_step_size: config.intitial_energy_step_size,
            wavefunction,
            last_diverge: 0.0,
        }
    }

    /// Popuplates the wavefunction vector with a solution to the Schrodinger equation
    /// and also determines the corresponding energy. The process requires iterating
    /// over many candidate energies and stopping when the energy step size becomes
    /// sufficiently small.
    fn solve(&mut self) {
        loop {
            self.compute_wavefunction();
            if self.energy_step_size.abs() <= self.config.energy_step_size_cutoff {
                break;
            }

            if self.wavefunction.last().unwrap() * self.last_diverge < 0.0 {
                self.energy_step_size = -self.energy_step_size / 2.0;
            }

            self.energy += self.energy_step_size;
            self.last_diverge = if self.wavefunction.last().unwrap() >= &0.0 {
                1.0
            } else {
                -1.0
            };
        }
    }

    fn energy(&self) -> f64 {
        self.energy
    }

    fn reset(&mut self) {
        *self = Self::new(&self.config);
    }

    fn wavefunction_points(&self) -> Vec<(f64, f64)> {
        let mut x_vals: Vec<f64> = Vec::new();
        let mut psi_vals: Vec<f64> = Vec::new();
        let mut pairs: Vec<(f64, f64)> = Vec::new();

        x_vals.push(-(self.wavefunction.len() as f64) * self.config.step_size);

        for val in self.wavefunction.iter().skip(1).rev() {
            x_vals.push(x_vals.last().unwrap() + self.config.step_size);
            psi_vals.push(match self.config.parity {
                Parity::Even => *val,
                Parity::Odd => -val,
            });
            pairs.push((*x_vals.last().unwrap(), *psi_vals.last().unwrap()));
        }
        for val in &self.wavefunction {
            x_vals.push(x_vals.last().unwrap() + self.config.step_size);
            psi_vals.push(*val);
            pairs.push((*x_vals.last().unwrap(), *psi_vals.last().unwrap()));
        }
        pairs
    }
}

/// The parity of solutions that a solver will look for when solving
/// the Schrodinger equation.
#[derive(Clone, Copy)]
pub enum Parity {
    Even,
    Odd,
}
