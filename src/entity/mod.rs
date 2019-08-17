use crate::entity::health::Health;
use crate::entity::vector::Vector;
use sfml::graphics::Sprite;

mod health;
pub mod vector;

#[derive(Default)]
pub struct Entity<'a>{
    pub ident: String,
    pub sprite: Sprite<'a>,
    pub health: Health,
    pub position: Vector,
}


fn create_entity(ident: String, sprite: Sprite, health: Option<f32>, position: Option<Vector>) -> Entity {
    let mut entity: Entity = Entity {ident, sprite, .. Entity::default()};

    if health.is_some() {
        entity.health.amount = health.unwrap();
        // Most likely if amount was specified, the entity can also take damage.
        entity.health.healable = true;
        entity.health.hurtable = true;
    }

    if position.is_some() {
        let pos = position.unwrap();
        entity.position.x = pos.x;
        entity.position.y = pos.y;
    }
    entity
}

pub fn create_player() -> Entity<'static> {
    create_entity("Player".to_string(), Sprite::new(), Option::Some(100.0), None)
}
