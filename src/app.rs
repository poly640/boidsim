use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crate::{
    sim::Sim,
    term::{clear_terminal, get_terminal_size, hide_cursor, move_cursor, show_cursor},
};

pub const REFRESH_RATE: u8 = 12;
pub const MARGIN_PERCENT_Y: f64 = 10.0;
pub const MARGIN_PERCENT_X: f64 = 15.0;

pub fn render(sim: &mut Sim) {
    // clear the terminal
    clear_terminal();

    // render boids
    for boid in sim.boids.iter() {
        move_cursor(boid.position.x as u16, boid.position.y as u16);
        print!("ඞ");
    }

    // render border
    let (w, h) = get_terminal_size();

    for i in 0..w {
        move_cursor(i, 0);
        print!("⣿");
        move_cursor(i, h);
        print!("⣿")
    }

    for i in 0..h {
        move_cursor(0, i);
        print!("⣿");
        move_cursor(w, i);
        print!("⣿")
    }

    // render margin
    /*let margin_x = ((MARGIN_PERCENT_X / 100.0) * w as f64) as u16;
    let margin_y = ((MARGIN_PERCENT_Y / 100.0) * h as f64) as u16;
    for i in margin_x..(w - margin_x) {
        move_cursor(i, margin_y);
        print!("⣿");
        move_cursor(i, h - margin_y);
        print!("⣿")
    }

    for i in margin_y..(h - margin_y) {
        move_cursor(margin_x, i);
        print!("⣿");
        move_cursor(w - margin_x, h - i);
        print!("⣿")
    }*/

    sim.frame += 1;

    // debug
    move_cursor(0, 0);
    print!(
        "Frame: {} FPS: {} Num Boids: {} ",
        sim.frame,
        REFRESH_RATE,
        sim.boids.len()
    );

    stdout().flush().expect("Could not flush output");
}

pub fn start() {
    let mut sim = Sim::new();
    hide_cursor();

    // main loop
    loop {
        sim.run();
        render(&mut sim);

        let sleep_time = ((1.0 / REFRESH_RATE as f64) * 1000.0) as u64;
        sleep(Duration::from_millis(sleep_time));
    }
}

pub fn stop() {
    show_cursor();
    stdout().flush().expect("Could not flush output");
    println!("Simulation finished");
}
