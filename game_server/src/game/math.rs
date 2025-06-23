use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec2F {
    x: f32,
    y: f32,
}

impl Vec2F {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    
    
}