extern crate sfml;
use crate::entity::{create_player};
use crate::entity::vector::Vector;

mod entity;

fn main() {
    let mut player = create_player();
}
