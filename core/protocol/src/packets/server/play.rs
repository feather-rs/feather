use super::*;

packets! {
    SpawnEntity {
        entity_id VarInt;
        uuid String;
        kind VarInt;
        x f64;
        y f64;
        z f64;

    }
}
