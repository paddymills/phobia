
mod model;
mod entity;

mod block;
mod extrude;
mod plane;
mod sketch;

pub use model::Model;
pub use entity::ModelEntity;
pub use extrude::Extrude;
pub use block::Block;
pub use plane::Plane;
pub use sketch::Sketch;


use crate::prelude::*;

pub trait ModelEntityObject {
    fn vertices(&self) -> Vec<Vertex>;
}

