pub mod app;
pub mod boid;
pub mod sim;
pub mod term;

fn main() {
    // println!("Hello, world!");
    app::start();
    //println!("Press enter to exit");
    //let _ = io::stdin().read_line(&mut String::from("")).unwrap();

    app::stop();
}
