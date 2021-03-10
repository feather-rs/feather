#[derive(Debug)]
pub enum HealthEventType {
    Regen(u32),

    FallDamage(u32),
    Hunger,
}
