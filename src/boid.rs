use vector2::Vector2;

#[derive(Debug)]
pub struct Boid {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Boid {
    pub fn new(pos: Option<Vector2>) -> Boid {
        Boid {
            position: pos.unwrap_or_else(|| Vector2::new(0.0, 0.0)),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}
