#![forbid(unsafe_code)]

//! Two implementations of a packet buffer.
//!
//! A packet buffer is used to hold the packets of a given type received
//! from players. When a packet is received, the IO threads
//! push the packet onto the buffer, and systems on the server
//! thread poll these packets out of the buffer.
//!
//! We provide two implementations, optimized for different cases:
//! * A buffer based on a large array, with two slots for each player.
//! This buffer works well for cases when packets of this type are received
//! very often, such as position updates.
//! * A buffer based on `crossbeam-channel`, best for cases where fewer
//! packets of this type are received.
//!
//! The former is not yet implemented, and we are currently using a `DashMap<Entity, SmallVec<[Box<dyn Packet>; 4]>>`.

use ahash::AHashMap;
use feather_core::network::{cast_packet, Packet, PacketType};
use fecs::Entity;
use indexmap::set::IndexSet;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use std::iter;
use strum::IntoEnumIterator;

/// The global packet store, storing packet buffers for packets of each type.
pub struct PacketBuffers {
    /// Packet buffers, indexed by the `ToPrimitive` implementation
    /// of `PacketType`.
    buffers: Vec<PacketBuffer>,
}

static USE_MAP_FOR: Lazy<IndexSet<PacketType>> = Lazy::new(|| {
    indexmap::indexset![
        PacketType::PlayerPosition,
        PacketType::PlayerPositionAndLookServerbound,
        PacketType::PlayerLook,
    ]
});

impl Default for PacketBuffers {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketBuffers {
    /// Creates a new packet store with buffers allocated for all packet types.
    pub fn new() -> Self {
        Self {
            buffers: PacketType::iter()
                .map(|ty| {
                    if USE_MAP_FOR.contains(&ty) {
                        PacketBuffer::Map(MapBuffer::default())
                    } else {
                        PacketBuffer::Channel(ChannelBuffer::new())
                    }
                })
                .collect(),
        }
    }

    /// Pushes a received packet onto the packet buffer for the packet's
    /// type.
    pub fn push(&self, entity: Entity, packet: Box<dyn Packet>) {
        let index = packet.ty().to_usize().unwrap();

        self.buffers[index].push(entity, packet);
    }

    /// Returns an iterator over packets received with type `T`.
    ///
    /// # Panics
    /// Panics if the underlying buffer is not a `ChannelBuffer`.
    /// `received_for()` should be used instead if using an `ArrayBuffer`.
    pub fn received<'a, T>(&'a self) -> impl Iterator<Item = (Entity, T)> + 'a
    where
        T: Packet,
    {
        let ty = T::ty_sized();

        let index = ty.to_usize().unwrap();

        self.buffers[index]
            .poll()
            .map(|(player, boxed)| (player, cast_packet(boxed)))
    }

    /// Returns an iterator over packets of type `T` received by the given player.
    ///
    /// # Panics
    /// Panics if the underlying buffer for this packet type is not a `MapBuffer` or an
    /// `ArrayBuffer`. Use `received` instead.
    pub fn received_for<T>(&self, player: Entity) -> impl Iterator<Item = T>
    where
        T: Packet,
    {
        let ty = T::ty_sized();

        let index = ty.to_usize().unwrap();

        self.buffers[index]
            .received_for(player)
            .map(|boxed| cast_packet(boxed))
    }
}

/// One of two packet buffer implementations.
pub enum PacketBuffer {
    Channel(ChannelBuffer),
    Map(MapBuffer),
}

impl PacketBuffer {
    /// Polls this buffer for newly received packets.
    ///
    /// # Panics
    /// Panics if the underlying buffer is not a `ChannelBuffer`.
    /// `received_for()` should be used instead if using an `ArrayBuffer`.
    pub fn poll<'a>(&'a self) -> impl Iterator<Item = (Entity, Box<dyn Packet>)> + 'a {
        match self {
            PacketBuffer::Channel(chan) => chan.poll(),
            PacketBuffer::Map(_) => panic!("cannot poll a map-based packet buffer"),
        }
    }

    /// Pushes a packet onto this buffer.
    pub fn push(&self, player: Entity, packet: Box<dyn Packet>) {
        match self {
            PacketBuffer::Channel(chan) => chan.push(player, packet),
            PacketBuffer::Map(map) => map.push(player, packet),
        }
    }

    /// Drains packets received by the given player.
    ///
    /// # Panics
    /// Panics if the underlying buffer is not a `MapBuffer` or an `ArrayBuffer`.
    pub fn received_for(&self, player: Entity) -> impl Iterator<Item = Box<dyn Packet>> {
        match self {
            PacketBuffer::Map(map) => map.received_for(player),
            PacketBuffer::Channel(_) => {
                panic!("cannot use received_for for a channel-based packet buffer")
            }
        }
    }
}

/// A packet buffer based on an MPMC channel. Best for packet types
/// which are received less frequently.
pub struct ChannelBuffer {
    sender: crossbeam::Sender<(Entity, Box<dyn Packet>)>,
    receiver: crossbeam::Receiver<(Entity, Box<dyn Packet>)>,
}

impl ChannelBuffer {
    fn new() -> Self {
        let (sender, receiver) = crossbeam::unbounded();
        Self { sender, receiver }
    }

    fn push(&self, player: Entity, packet: Box<dyn Packet>) {
        let _ = self.sender.send((player, packet));
    }

    fn poll<'a>(&'a self) -> impl Iterator<Item = (Entity, Box<dyn Packet>)> + 'a {
        self.receiver.try_iter()
    }
}

enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B, I> Iterator for Either<A, B>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(a) => a.next(),
            Either::Right(b) => b.next(),
        }
    }
}

type MapBufferVec = SmallVec<[Box<dyn Packet>; 2]>;
type MapBufferInner = AHashMap<Entity, Mutex<MapBufferVec>>;

#[derive(Default)]
pub struct MapBuffer(RwLock<MapBufferInner>);

impl MapBuffer {
    fn push(&self, player: Entity, packet: Box<dyn Packet>) {
        let guard = self.0.read();
        if let Some(vec) = guard.get(&player) {
            vec.lock().push(packet);
        } else {
            drop(guard);
            self.0
                .write()
                .insert(player, Mutex::new(std::iter::once(packet).collect()));
        }
    }

    fn received_for(&self, player: Entity) -> impl Iterator<Item = Box<dyn Packet>> {
        let map_guard = self.0.read();

        if let Some(vec) = map_guard.get(&player) {
            let vec = vec
                .lock()
                .drain(..)
                .collect::<SmallVec<[Box<dyn Packet>; 2]>>();

            Either::Left(vec.into_iter())
        } else {
            Either::Right(iter::empty())
        }
    }
}

/* TODO: audit this implementation.
/// A packet buffer using an array of slots.
pub struct ArrayBuffer {
    /// Internal array of length `2 * (num_players rounded up to the next power of two)`.
    /// Packets received for a player with index `i` will
    /// be located at `array[i]` and `array[n + i]`, where `n` is the number
    /// of players rounded up to the next power of two.
    ///
    /// Note that this array is type-erased; we do this as an optimization
    /// to store the packets directly in the array instead of going through a
    /// `Box`.
    array: RwLock<NonNull<u8>>,
    /// Memory layout of `array`.
    array_layout: Mutex<Layout>,
    /// Layout of a single packet.
    single_packet: Layout,
    /// Number of players for which this buffer has capacity.
    max_players: AtomicUsize,
    /// Pointer to the `None` value for the packet.
    none_ptr: NonNull<u8>,
    /// Length of the `None` value for the packet.
    none_len: usize,
}

impl ArrayBuffer {
    /// Creates a new, empty `ArrayBuffer` for packets of type `T`.
    pub fn new<T>() -> Self {
        let starting_n = 8;

        let none = Box::new(Option::<T>::None);
        let none_ptr = NonNull::new(Box::into_raw(none).cast()).expect("box has null pointer");

        let (array, array_layout) =
            unsafe { Self::allocate_for(Layout::new::<Option<T>>(), starting_n, none_ptr) };

        Self {
            array: RwLock::new(array),
            array_layout: Mutex::new(array_layout),
            single_packet: Layout::new::<Option<T>>(),
            max_players: AtomicUsize::new(starting_n),
            none_ptr,
            none_len: std::mem::size_of::<Option<T>>(),
        }
    }

    /// Returns the number of players supported by this buffer.
    pub fn max_players(&self) -> usize {
        self.max_players.load(Ordering::Acquire)
    }

    /// Extends this array buffer to support at least `n` __more__ players.
    pub fn reserve(&self, extra: usize) {
        let mut old_array = self.array.write();
        let mut old_layout = self.array_layout.lock();

        let current_max = self.max_players.load(Ordering::Acquire);
        let new_max = (current_max + extra).next_power_of_two();

        let (new_array, new_layout) =
            unsafe { Self::allocate_for(self.single_packet, new_max, self.none_ptr) };

        assert!(new_layout.size() > old_layout.size());
        assert_eq!(new_layout.size() % 2, 0);
        assert_eq!(old_layout.size() % 2, 0);

        // copy existing packets to new array
        unsafe {
            std::ptr::copy_nonoverlapping(
                old_array.as_ptr(),
                new_array.as_ptr(),
                old_layout.size() / 2,
            );
            std::ptr::copy_nonoverlapping(
                old_array.as_ptr().offset((old_layout.size() / 2) as isize),
                new_array.as_ptr().offset((new_layout.size() / 2) as isize),
                old_layout.size() / 2,
            );
        }

        *old_array = new_array;
        *old_layout = new_layout;
        self.max_players.store(new_max, Ordering::Release);
    }

    unsafe fn allocate_for(
        single_packet: Layout,
        max_players: usize,
        none_ptr: NonNull<u8>,
    ) -> (NonNull<u8>, Layout) {
        let new_layout = single_packet
            .repeat(max_players * 2)
            .map(|(layout, offset)| {
                assert_eq!(offset, single_packet.size());
                layout
            })
            .expect("invalid packet buffer layout");

        let new_array =
            NonNull::new(std::alloc::alloc(new_layout)).expect("allocator returned null pointer");

        // fill array with `None` values
        for i in 0..max_players * 2 {
            let ptr = new_array
                .as_ptr()
                .offset((i * single_packet.size()) as isize);
            std::ptr::copy_nonoverlapping(none_ptr.as_ptr(), ptr, single_packet.size());
        }

        (new_array, new_layout)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::network::packets::Request;
    use fecs::{EntityBuilder, World};

    #[test]
    fn map_buffer() {
        let buffer = MapBuffer::default();

        let mut world = World::new();
        let entity = EntityBuilder::new().build().spawn_in(&mut world);

        dbg!();
        buffer.push(entity, Box::new(Request {}));
        dbg!();

        let mut received = buffer.received_for(entity).collect::<Vec<_>>();
        dbg!();

        assert_eq!(received.len(), 1);

        let first = received.remove(0);
        let _ = cast_packet::<Request>(first);
    }

    #[test]
    fn channel_buffer() {
        let buffer = ChannelBuffer::new();

        let mut world = World::new();
        let entity = EntityBuilder::new().build().spawn_in(&mut world);

        buffer.push(entity, Box::new(Request {}));

        let mut received = buffer.poll().collect::<Vec<_>>();

        assert_eq!(received.len(), 1);

        let (rentity, first) = received.remove(0);
        let _ = cast_packet::<Request>(first);

        assert_eq!(entity, rentity);
    }
}
