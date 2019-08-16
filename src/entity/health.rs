pub struct Health {
    pub amount: f32,
    pub healable: bool,
    pub hurtable: bool,
}

impl Default for Health{
    fn default() -> Self {
        // Default to this, since most entities are walls.
        Health {amount: 0.0, healable: false, hurtable: false}
    }
}