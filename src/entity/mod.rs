use crate::entity::health::Health;

mod health;

#[derive(Default)]
pub struct Entity {
    pub ident: String,
    pub health: Health,
}

fn create_entity(name: String, health: Option<f32>) -> Entity {
    let mut entity: Entity = Default::default();

    if health.is_some() {
        entity.health.amount = health.unwrap();
        // Most likely if amount was specified, the entity can also take damage.
        entity.health.healable = true;
        entity.health.hurtable = true;
    }
    entity
}

pub fn create_player() -> Entity {
    create_entity("Player".to_string(), Option::Some(100.0))
}
