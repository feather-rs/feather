#[derive(Debug)]
pub enum HealthEventType {
    /// How many hearts should be regenerated.
    Regen(u32),

    /// How many blocks an entity has fallen.
    FallDamage(u32),
    Hunger,
}
