//! Defines the components available to Quill plugins.

use std::{any::TypeId, borrow::Cow as CloneOnWrite};

use libcraft_core::{Gamemode, Position};
use libcraft_particles::Particle;
use uuid::Uuid;

use crate::components::*;
use crate::entities::*;
use crate::events::*;

/// Used to convert dynamic `HostComponent`s to
/// statically-typed generic `T`s.
///
/// Use with [`HostComponent::visit`].
pub trait ComponentVisitor<R> {
    fn visit<T: Component>(self) -> R;
}

/// Generates the [`HostComponent`] enum.
///
/// Adds a method `type_id` that returns the TypeId
/// of the component's type. This is used on the
/// host to construct queries.
macro_rules! host_component_enum {
    (
        $(#[$outer:meta])*
        pub enum $ident:ident {
            $(
                $component:ident = $x:literal
            ),* $(,)?
        }
    ) => {
        c_enum! {
            $(#[$outer])*
            pub enum $ident {
                $($component = $x,)*
            }
        }

        impl $ident {
            pub fn type_id(self) -> TypeId {
                match self {
                    $(Self::$component => TypeId::of::<$component>(),)*
                }
            }

            /// Invokes a `ComponentVisitor`'s `visit`
            /// method where the type `T` is the type of this component.
            pub fn visit<R>(self, visitor: impl ComponentVisitor<R>) -> R {
                match self {
                    $(Self::$component => visitor.visit::<$component>(),)*
                }
            }
        }
    }
}

host_component_enum! {
    /// A component that is stored on the host
    /// and accessible from plugins.
    pub enum HostComponent {
        // `Pod` components
        Position = 0,

        // Entity marker components
        AreaEffectCloudMarker = 100,
        ArmorStandMarker = 101,
        ArrowMarker = 102,
        BatMarker = 103,
        BeeMarker = 104,
        BlazeMarker = 105,
        BoatMarker = 106,
        CatMarker = 107,
        CaveSpiderMarker = 108,
        ChickenMarker = 109,
        CodMarker = 110,
        CowMarker = 111,
        CreeperMarker = 112,
        DolphinMarker = 113,
        DonkeyMarker = 114,
        DragonFireballMarker = 115,
        DrownedMarker = 116,
        ElderGuardianMarker = 117,
        PiglinBruteMarker = 118,
        EndCrystalMarker = 119,
        EnderDragonMarker = 120,
        EndermanMarker = 121,
        EndermiteMarker = 122,
        EvokerMarker = 123,
        EvokerFangsMarker = 124,
        ExperienceOrbMarker = 125,
        EyeOfEnderMarker = 126,
        FallingBlockMarker = 127,
        FireworkRocketMarker = 128,
        FoxMarker = 129,
        GhastMarker = 130,
        GiantMarker = 131,
        GuardianMarker = 132,
        HoglinMarker = 133,
        HorseMarker = 134,
        HuskMarker = 135,
        IllusionerMarker = 136,
        IronGolemMarker = 137,
        ItemMarker = 138,
        ItemFrameMarker = 139,
        FireballMarker = 140,
        LeashKnotMarker = 141,
        LightningBoltMarker = 142,
        LlamaMarker = 143,
        LlamaSpitMarker = 144,
        MagmaCubeMarker = 145,
        MinecartMarker = 146,
        ChestMinecartMarker = 147,
        CommandBlockMinecartMarker = 148,
        FurnaceMinecartMarker = 149,
        HopperMinecartMarker = 150,
        SpawnerMinecartMarker = 151,
        TntMinecartMarker = 152,
        MuleMarker = 153,
        MooshroomMarker = 154,
        OcelotMarker = 155,
        PaintingMarker = 156,
        PandaMarker = 157,
        ParrotMarker = 158,
        PhantomMarker = 159,
        PigMarker = 160,
        PiglinMarker = 161,
        PillagerMarker = 162,
        PolarBearMarker = 163,
        TntMarker = 164,
        PufferfishMarker = 165,
        RabbitMarker = 166,
        RavagerMarker = 167,
        SalmonMarker = 168,
        SheepMarker = 169,
        ShulkerMarker = 170,
        ShulkerBulletMarker = 171,
        SilverfishMarker = 172,
        SkeletonMarker = 173,
        SkeletonHorseMarker = 174,
        SlimeMarker = 175,
        SmallFireballMarker = 176,
        SnowGolemMarker = 177,
        SnowballMarker = 178,
        SpectralArrowMarker = 179,
        SpiderMarker = 180,
        SquidMarker = 181,
        StrayMarker = 182,
        StriderMarker = 183,
        EggMarker = 184,
        EnderPearlMarker = 185,
        ExperienceBottleMarker = 186,
        PotionMarker = 187,
        TridentMarker = 188,
        TraderLlamaMarker = 189,
        TropicalFishMarker = 190,
        TurtleMarker = 191,
        VexMarker = 192,
        VillagerMarker = 193,
        VindicatorMarker = 194,
        WanderingTraderMarker = 195,
        WitchMarker = 196,
        WitherMarker = 197,
        WitherSkeletonMarker = 198,
        WitherSkullMarker = 199,
        WolfMarker = 200,
        ZoglinMarker = 201,
        ZombieMarker = 202,
        ZombieHorseMarker = 203,
        ZombieVillagerMarker = 204,
        ZombifiedPiglinMarker = 205,
        PlayerMarker = 206,
        FishingBobberMarker = 207,

        // `bincode` components
        Gamemode = 1000,
        Uuid = 1001,
        OnGround = 1002,
        Name = 1003,
        CustomName = 1004,
        Particle = 1005,
        InteractEntityEvent = 1006,
        BlockPlacementEvent = 1007,
        BlockInteractEvent = 1008,
        CreativeFlying = 1009,
        CreativeFlyingEvent = 1010,
        Sneaking = 1011,
        SneakEvent = 1012,
        Sprinting = 1013,
        SprintEvent = 1014,


    }
}

/// How a component will be serialized.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SerializationMethod {
    /// Copy raw bytes with `bytemuck`.
    Bytemuck,
    /// Serialize into a `Vec` with `bincode.
    Bincode,
}

/// A type that can be used as a component.
///
/// # Safety
/// [`Component::from_bytes`] must return `Some(_)` if given
/// any byte slice returned by [`Component::to_bytes`]. A violation
/// if this contract may result in undefined behavior
/// on the plugin side.
pub unsafe trait Component: Send + Sync + Sized + 'static {
    /// How this component will be serialized.
    const SERIALIZATION_METHOD: SerializationMethod;

    /// Returns the [`HostComponent`] corresponding to this
    /// component.
    ///
    /// # Contract
    /// A sound implementation of this method _must_
    /// return the `HostComponent` corresponding to
    /// this type.
    fn host_component() -> HostComponent;

    /// Serializes this component to bytes suitable
    /// for deserialization by `from_bytes`.
    ///
    /// Should panic if `Self::SERIALIZATION_METHOD == SerializationMethod::Bytemuck`.
    fn to_bytes(&self, target: &mut Vec<u8>);

    /// Gets this component as a byte slice.
    ///
    /// Should panic if `Self::SERIALIZATION_METHOD != SerializationMethod::Bytemuck`.
    fn as_bytes(&self) -> &[u8];

    /// Deserializes this component from bytes
    /// returned by [`Component::to_bytes`].
    ///
    /// Returns the number of bytes used to deserialize
    /// `self`. `bytes` may have a greater length than is needed.
    ///
    /// # Contract
    /// A sound implementation of this method _must_
    /// return `Some(_)` for all values of `bytes`
    /// that can be returned by `Self::to_bytes`.
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)>;

    /// Deserializes this component from bytes returned by [`Component::to_bytes`]
    /// without validating correctness.
    ///
    /// The default implementation of this method calls [`Self::from_bytes`]
    /// and then performs an unchecked unwrap.
    ///
    /// # Safety
    /// Behavior is undefined if `bytes` was not previously
    /// returned from a call to `Self::to_bytes`.
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> (Self, usize) {
        // Do an unchecked unwrap of `from_bytes`.
        // This should cause the optimizer to
        // remove safety checks in `bincode`,
        // which may improve performance on the plugin side.
        match Self::from_bytes(bytes) {
            Some(this) => this,
            None => std::hint::unreachable_unchecked(),
        }
    }

    /// Serializes `self` into bytes using
    /// the appropriate `SerializationMethod`.
    fn to_cow_bytes(&self) -> CloneOnWrite<[u8]> {
        match Self::SERIALIZATION_METHOD {
            SerializationMethod::Bytemuck => CloneOnWrite::Borrowed(self.as_bytes()),
            SerializationMethod::Bincode => {
                let mut buffer = Vec::new();
                self.to_bytes(&mut buffer);
                CloneOnWrite::Owned(buffer)
            }
        }
    }
}

macro_rules! pod_component_impl {
    ($type:ident) => {
        unsafe impl crate::component::Component for $type {
            const SERIALIZATION_METHOD: crate::component::SerializationMethod =
                crate::component::SerializationMethod::Bytemuck;

            fn host_component() -> crate::component::HostComponent {
                crate::component::HostComponent::$type
            }

            fn to_bytes(&self, _target: &mut Vec<u8>) {
                unreachable!()
            }

            fn as_bytes(&self) -> &[u8] {
                bytemuck::cast_slice(std::slice::from_ref(self))
            }

            fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
                let this = bytemuck::try_from_bytes(&bytes[..std::mem::size_of::<Self>()])
                    .ok()
                    .copied()?;
                Some((this, std::mem::size_of::<Self>()))
            }
        }
    };
}

pod_component_impl!(Position);

/**
If you are using this macro and you get the error:
```
    error[E0599]: no variant or associated item named `...` found for enum `HostComponent` in the current scope.
```
Then you need to go to the top of the file were this macro is defined. There you find the HostCompoent enum, that
you need to add your component to.
*/
macro_rules! bincode_component_impl {
    ($type:ident) => {
        unsafe impl crate::Component for $type {
            const SERIALIZATION_METHOD: crate::component::SerializationMethod =
                crate::component::SerializationMethod::Bincode;

            fn host_component() -> crate::component::HostComponent {
                crate::component::HostComponent::$type
            }

            fn to_bytes(&self, target: &mut Vec<u8>) {
                bincode::serialize_into(target, self).expect("failed to serialize component");
            }

            fn as_bytes(&self) -> &[u8] {
                unreachable!()
            }

            fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
                let mut cursor = std::io::Cursor::new(bytes);
                let this = bincode::deserialize_from(&mut cursor).ok()?;
                Some((this, cursor.position() as usize))
            }
        }
    };
}

bincode_component_impl!(Gamemode);
bincode_component_impl!(Uuid);
bincode_component_impl!(Particle);
bincode_component_impl!(InteractEntityEvent);
bincode_component_impl!(BlockPlacementEvent);
bincode_component_impl!(BlockInteractEvent);
bincode_component_impl!(CreativeFlyingEvent);
bincode_component_impl!(SneakEvent);
bincode_component_impl!(SprintEvent);
