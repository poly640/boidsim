use std::vec;

use rand::Rng;
use vector2::Vector2;

use crate::{
    app::{MARGIN_PERCENT_X, MARGIN_PERCENT_Y, REFRESH_RATE},
    boid::Boid,
    term::get_terminal_size,
};

const NUM_BOIDS: u16 = 500;
const SIM_SPEED: f64 = 10.0;

const SAFE_RANGE: f64 = 2.0;
const GROUP_RANGE: f64 = 30.0;

const AVOID_FACTOR: f64 = 0.05 * SIM_SPEED;
const MATCHING_FACTOR: f64 = 0.05 * SIM_SPEED;
const CENTER_FACTOR: f64 = 0.0005 * SIM_SPEED;
const TURN_FACTOR: f64 = 0.3 * SIM_SPEED;

const MIN_SPEED: f64 = 4.0 * SIM_SPEED;
const MAX_SPEED: f64 = 8.0 * SIM_SPEED;

pub struct Sim {
    pub boids: Vec<Boid>,
    pub frame: u64,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            boids: vec![],
            frame: 0,
        }
    }

    pub fn add_boid(&mut self, boid: Boid) {
        self.boids.push(boid);
    }

    pub fn run(&mut self) {
        let (w, h) = get_terminal_size();

        // Spawn boids
        if self.boids.len() < NUM_BOIDS as usize {
            let mut boid = Boid::new(Some(Vector2::new((w / 2) as f64, (h / 2) as f64)));
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);

            boid.velocity = Vector2::new(x, y);

            self.add_boid(boid);
        }

        // Simulate boids
        for i in 0..self.boids.len() {
            let mut pos = self.boids[i].position;
            let mut vel = self.boids[i].velocity;

            let mut avg_vel = Vector2::ZERO;
            let mut avg_pos = Vector2::ZERO;
            let mut neighbors: u16 = 0;

            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                };

                let other_boid = &self.boids[j];
                let dist = (pos - other_boid.position).magnitude();
                if dist < SAFE_RANGE {
                    // Separation
                    vel += (pos - other_boid.position) * AVOID_FACTOR;
                }

                if dist < GROUP_RANGE {
                    // Add to average
                    avg_pos += other_boid.position;
                    avg_vel += other_boid.velocity;
                    neighbors += 1;
                }
            }

            // Alignment & Cohesion
            if neighbors > 0 {
                avg_pos /= neighbors as f64;
                avg_vel /= neighbors as f64;
                vel += (avg_vel - vel) * MATCHING_FACTOR;
                vel += (avg_pos - pos) * CENTER_FACTOR;
            }

            // Screen bounds
            let margin_x = (MARGIN_PERCENT_X / 100.0) * w as f64;
            let margin_y = (MARGIN_PERCENT_Y / 100.0) * h as f64;

            let turn_x = match pos.x {
                x if x < margin_x => TURN_FACTOR,
                x if x > (w as f64 - margin_x) => -TURN_FACTOR,
                _ => 0.0,
            };

            let turn_y = match pos.y {
                y if y < margin_y => TURN_FACTOR,
                y if y > (h as f64 - margin_y) => -TURN_FACTOR,
                _ => 0.0,
            };

            vel += Vector2::new(turn_x, turn_y);

            // Speed clamp
            let speed = vel.magnitude();
            if speed > MAX_SPEED {
                vel = (vel / speed) * MAX_SPEED;
            } else if speed < MIN_SPEED {
                vel = (vel / speed) * MIN_SPEED;
            }

            // Apply velocity
            let delta = vel / REFRESH_RATE as f64;
            pos += delta;

            // Position clamp
            pos = Vector2::new(
                pos.x.clamp(1.0, (w as f64) - 2.0),
                pos.y.clamp(1.0, (h as f64) - 2.0),
            );

            // Update
            let main_boid = &mut self.boids[i];
            main_boid.velocity = vel;
            main_boid.position = pos;
        }
    }
}
