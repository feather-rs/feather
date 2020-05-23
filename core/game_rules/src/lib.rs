macro_rules! gamerules {
    [$($name:ident($value:ty) = $default:literal),*$(,)*] => {
        #[allow(non_snake_case)]
        #[derive(Default)]
        pub struct GameRules {
            $(
                pub $name: $name
            ),*
        }

        $(
            #[derive(Debug, PartialEq, Eq)]
            pub struct $name($value);

            impl Default for $name {
                fn default() -> Self {
                    Self($default)
                }
            }

            impl AsRef<$value> for $name {
                fn as_ref(&self) -> &$value {
                    &self.0
                }
            }

            impl std::ops::Deref for $name {
                type Target = $value;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        )*
    };
}

gamerules![
    AnnounceAdvancements(bool) = true,
    CommandBlockOutput(bool) = true,
    DisableElytraMovementCheck(bool) = false,
    DisableRaids(bool) = false,
    DoDaylightCycle(bool) = true,
    DoEntityDrops(bool) = true,
    DoFireTick(bool) = true,
    DoInsomnia(bool) = true,
    DoImmediateRespawn(bool) = false,
    DoLimitedCrafting(bool) = false,
    DoMobLoot(bool) = true,
    DoMobSpawning(bool) = true,
    DoPatrolSpawning(bool) = true,
    DoTileDrops(bool) = true,
    DoTraderSpawning(bool) = true,
    DoWeatherCycle(bool) = true,
    DrowningDamage(bool) = true,
    FallDamage(bool) = true,
    FireDamage(bool) = true,
    KeepInventory(bool) = false,
    LogAdminCommands(bool) = true,
    MaxCommandChainLength(u32) = 65536,
    MaxEntityCramming(u32) = 24,
    MobGriefing(bool) = true,
    NaturalRegeneration(bool) = true,
    RandomTickSpeed(u32) = 3,
    ReducedDebugInfo(bool) = false,
    SendCommandFeedback(bool) = true,
    ShowDeathMessages(bool) = true,
    SpawnRadius(u32) = 10,
    SpectatorsGenerateChunks(bool) = true,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let rules = GameRules::default();
        assert_eq!(rules.AnnounceAdvancements, AnnounceAdvancements::default());
    }
}
