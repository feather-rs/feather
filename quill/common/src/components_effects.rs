use libcraft_effects::{effects::EffectApplication, Effect};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Effects(pub BTreeSet<EffectApplication>);
impl Effects {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_effect(&mut self, effect: EffectApplication) {
        self.0.insert(effect);
    }
    pub fn strongest_active(&self, kind: Effect) -> Option<&EffectApplication> {
        self.0.iter().filter(|e| e.kind == kind).last()
    }
}
bincode_component_impl!(Effects);
/* 
macro_rules! impl_effect {
    ($ident:ident) => {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
        pub struct $ident(pub BTreeSet<EffectApplication>);
        impl $ident {
            pub fn new() -> $ident {
                $ident { 0: BTreeSet::new() }
            }
            pub fn add_effect(&mut self, effect: EffectApplication) -> bool {
                self.0.insert(effect)
            }
            pub fn not_started(&mut self) -> Vec<EffectApplication> {
                self.0
                    .iter()
                    .filter(|effect| effect.start_tick == 0)
                    .cloned()
                    .collect::<Vec<_>>()
            }
            pub fn ended_on_tick(&mut self, tick: u64) -> Vec<EffectApplication> {
                self.0
                    .iter()
                    .filter(|effect| {
                        tick >= effect.start_tick + effect.duration as u64 && effect.start_tick != 0
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            }
            pub fn active_effect(&mut self) -> Option<&EffectApplication> {
                self.0.iter().last()
            }
        }
        impl Default for $ident {
            fn default() -> Self {
                $ident::new()
            }
        }
        bincode_component_impl!($ident);
    };
}


impl_effect!(SpeedEffect);
impl_effect!(SlownessEffect);
impl_effect!(HasteEffect);
impl_effect!(MiningFatigueEffect);
impl_effect!(StrengthEffect);
impl_effect!(InstantHealthEffect);
impl_effect!(InstantDamageEffect);
impl_effect!(JumpBoostEffect);
impl_effect!(NauseaEffect);
impl_effect!(RegenerationEffect);
impl_effect!(ResistanceEffect);
impl_effect!(FireResistanceEffect);
impl_effect!(WaterBreathingEffect);
impl_effect!(InvisibilityEffect);
impl_effect!(BlindnessEffect);
impl_effect!(NightVisionEffect);
impl_effect!(HungerEffect);
impl_effect!(WeaknessEffect);
impl_effect!(PoisonEffect);
impl_effect!(WitherEffect);
impl_effect!(HealthBoostEffect);
impl_effect!(AbsorptionEffect);
impl_effect!(SaturationEffect);
impl_effect!(GlowingEffect);
impl_effect!(LevitationEffect);
impl_effect!(LuckEffect);
impl_effect!(BadLuckEffect);
impl_effect!(SlowFallingEffect);
impl_effect!(ConduitPowerEffect);
impl_effect!(DolphinsGraceEffect);
impl_effect!(BadOmenEffect);
impl_effect!(HeroOfTheVillageEffect);
*/
/// A walk speed modifier in percent
#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct WalkEffectModifier(pub BTreeMap<Effect, i32>);

impl WalkEffectModifier {
    pub fn new() -> WalkEffectModifier {
        WalkEffectModifier { 0: BTreeMap::new() }
    }
}

impl Default for WalkEffectModifier {
    fn default() -> Self {
        WalkEffectModifier::new()
    }
}

bincode_component_impl!(WalkEffectModifier);
