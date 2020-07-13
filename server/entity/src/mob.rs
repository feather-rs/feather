//! Components and functionality shared across all mobs.

mod boss;
mod defensive;
mod hostile;
mod neutral;
mod passive;

pub use boss::*;
pub use defensive::*;
use feather_core::entitymeta::EntityMetadata;
use feather_core::network::packets::SpawnMob;
use feather_core::network::Packet;
use feather_core::util::Position;
use feather_server_types::{NetworkId, SpawnPacketCreator, Uuid, Velocity};
use feather_server_util::{degrees_to_stops, protocol_velocity};
use fecs::{EntityBuilder, EntityRef};
pub use hostile::*;
pub use neutral::*;
pub use passive::*;

/// Enumeration of mob types. Note that this enum should not be
/// used in queries to identify mobs of a given type.
///
/// This is _not_ a component. It is only used for utility
/// functions such as `mob::spawn_packet_creator`.
///
/// https://wiki.vg/Entity_metadata#Mobs
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MobKind {
    Bat = 3,
    Blaze = 4,
    CaveSpider = 6,
    Chicken = 7,
    Cod = 8,
    Cow = 9,
    Creeper = 10,
    Donkey = 11,
    Dolphin = 12,
    Drowned = 14,
    ElderGuardian = 15,
    EnderDragon = 16,
    Enderman = 18,
    Endermite = 19,
    EvocationIllager = 21,
    Ghast = 26,
    Giant = 27,
    Guardian = 28,
    Horse = 29,
    Husk = 30,
    IllusionIllager = 31,
    Llama = 36,
    MagmaCube = 38,
    Mule = 46,
    MushroomCow = 47,
    Ocelot = 48,
    Parrot = 50,
    Pig = 51,
    Pufferfish = 52,
    PigZombie = 53,
    PolarBear = 54,
    Rabbit = 56,
    Salmon = 57,
    Sheep = 58,
    Shulker = 59,
    Silverfish = 61,
    Skeleton = 62,
    SkeletonHorse = 63,
    Slime = 64,
    SnowGolem = 66,
    Spider = 69,
    Squid = 70,
    Stray = 71,
    TropicalFish = 72,
    Turtle = 73,
    Vex = 78,
    Villager = 79,
    IronGolem = 80,
    VindicationIllager = 81,
    Witch = 82,
    Wither = 83,
    WitherSkeleton = 84,
    Wolf = 86,
    Zombie = 87,
    ZombieHorse = 88,
    ZombieVillager = 89,
    Phantom = 90,
}

/// Returns the base components for a mob with the given
/// kind.
pub fn base(kind: MobKind) -> EntityBuilder {
    super::base().with(spawn_packet_creator(kind))
}

/// Returns a `SpawnPacketCreator` for a mob with the given kind.
pub fn spawn_packet_creator(kind: MobKind) -> SpawnPacketCreator {
    let f = Box::new(move |accessor: &EntityRef| {
        let entity_id = accessor.get::<NetworkId>().0;
        let uuid = accessor
            .try_get::<Uuid>()
            .map(|r| *r)
            .unwrap_or_else(Uuid::new_v4);

        let position = *accessor.get::<Position>();
        let velocity = *accessor.get::<Velocity>();
        let meta = accessor
            .try_get::<EntityMetadata>()
            .map(|meta| (&*meta).clone())
            .unwrap_or_else(EntityMetadata::entity_base);

        let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

        let res: Box<dyn Packet> = Box::new(SpawnMob {
            entity_id,
            entity_uuid: uuid,
            ty: kind as i32,
            x: position.x,
            y: position.y,
            z: position.z,
            yaw: degrees_to_stops(position.yaw),
            pitch: degrees_to_stops(position.pitch),
            head_pitch: 0, // todo
            velocity_x,
            velocity_y,
            velocity_z,
            meta,
        });
        res
    });

    SpawnPacketCreator(Box::leak(f))
}
