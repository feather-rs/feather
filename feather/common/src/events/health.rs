/// Raw events for triggering the respective events on `Health`.
/// Only to be consumed by `health_events_handler`.
#[derive(Debug)]
pub enum HealthEventType {
    Heal(u32),
    Harm(u32),
}

#[derive(Debug)]
pub struct FallDamage(pub u32);

#[derive(Debug)]
pub struct ExplosionDamage(pub u32);

#[derive(Debug)]
pub struct FireDamage;

#[derive(Debug)]
pub struct SoulFireDamage;

#[derive(Debug)]
pub struct PoisonDamage;
