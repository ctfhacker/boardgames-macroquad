use macroquad::*;

pub mod row;
pub mod piece;
pub mod assets;

pub trait Resizeable {
    /// Draws the element at the given `location` resized using `adjustment`
    fn draw(&self, location: Vec2, adjustment: f32);
}
