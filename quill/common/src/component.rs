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
        AreaEffectCloud = 100,
        ArmorStand = 101,
        Arrow = 102,
        Bat = 103,
        Bee = 104,
        Blaze = 105,
        Boat = 106,
        Cat = 107,
        CaveSpider = 108,
        Chicken = 109,
        Cod = 110,
        Cow = 111,
        Creeper = 112,
        Dolphin = 113,
        Donkey = 114,
        DragonFireball = 115,
        Drowned = 116,
        ElderGuardian = 117,
        EndCrystal = 118,
        EnderDragon = 119,
        Enderman = 120,
        Endermite = 121,
        Evoker = 122,
        EvokerFangs = 123,
        ExperienceOrb = 124,
        EyeOfEnder = 125,
        FallingBlock = 126,
        FireworkRocket = 127,
        Fox = 128,
        Ghast = 129,
        Giant = 130,
        Guardian = 131,
        Hoglin = 132,
        Horse = 133,
        Husk = 134,
        Illusioner = 135,
        IronGolem = 136,
        Item = 137,
        ItemFrame = 138,
        Fireball = 139,
        LeashKnot = 140,
        LightningBolt = 141,
        Llama = 142,
        LlamaSpit = 143,
        MagmaCube = 144,
        Minecart = 145,
        ChestMinecart = 146,
        CommandBlockMinecart = 147,
        FurnaceMinecart = 148,
        HopperMinecart = 149,
        SpawnerMinecart = 150,
        TntMinecart = 151,
        Mule = 152,
        Mooshroom = 153,
        Ocelot = 154,
        Painting = 155,
        Panda = 156,
        Parrot = 157,
        Phantom = 158,
        Pig = 159,
        Piglin = 160,
        Pillager = 161,
        PolarBear = 162,
        Tnt = 163,
        Pufferfish = 164,
        Rabbit = 165,
        Ravager = 166,
        Salmon = 167,
        Sheep = 168,
        Shulker = 169,
        ShulkerBullet = 170,
        Silverfish = 171,
        Skeleton = 172,
        SkeletonHorse = 173,
        Slime = 174,
        SmallFireball = 175,
        SnowGolem = 176,
        Snowball = 177,
        SpectralArrow = 178,
        Spider = 179,
        Squid = 180,
        Stray = 181,
        Strider = 182,
        Egg = 183,
        EnderPearl = 184,
        ExperienceBottle = 185,
        Potion = 186,
        Trident = 187,
        TraderLlama = 188,
        TropicalFish = 189,
        Turtle = 190,
        Vex = 191,
        Villager = 192,
        Vindicator = 193,
        WanderingTrader = 194,
        Witch = 195,
        Wither = 196,
        WitherSkeleton = 197,
        WitherSkull = 198,
        Wolf = 199,
        Zoglin = 200,
        Zombie = 201,
        ZombieHorse = 202,
        ZombieVillager = 203,
        ZombifiedPiglin = 204,
        Player = 205,
        FishingBobber = 206,
        PiglinBrute = 207,

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
        PreviousGamemode = 1015,
        Health = 1016,
        WalkSpeed = 1017,
        CreativeFlyingSpeed = 1018,
        CanCreativeFly = 1019,
        CanBuild = 1020,
        Instabreak = 1021,
        Invulnerable = 1022,
        PlayerJoinEvent = 1023,
        EntityRemoveEvent = 1024,
        EntityCreateEvent = 1025,
        GamemodeEvent = 1026,
        InstabreakEvent = 1027,
        FlyingAbilityEvent = 1028,
        BuildingAbilityEvent = 1029,
        InvulnerabilityEvent = 1030,
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
bincode_component_impl!(PlayerJoinEvent);
bincode_component_impl!(EntityRemoveEvent);
bincode_component_impl!(EntityCreateEvent);
bincode_component_impl!(GamemodeEvent);
bincode_component_impl!(InstabreakEvent);
bincode_component_impl!(FlyingAbilityEvent);
bincode_component_impl!(BuildingAbilityEvent);
bincode_component_impl!(InvulnerabilityEvent);
