# ECS
ECS stands for Entity Component System and is an integral part of feather and quill.

ECS is an alternative to Object Oriented Programming where you usally stores data in a object and then arrays of the object.
In ecs you store each field as an array and combine the data into an object at the end. 


A `Component` is either a marker or a small amount of data.

```rust
pub struct PlayerMarker;
pub struct PiglinMarker;
pub struct MessageReciver;
```

```rust
pub struct Health {
    pub value: u32,
    pub max: u32,
}
```


```rust
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
} 
```

Maybe inventories should be split up into many smaller inventories?
```rust
pub struct Inventory {

}

pub struct Armor {

}

pub struct Helmet(ItemStack)

```

An example of an `enum` component.

An `Entity` is an id which refrences a set of components stored in the `Ecs`. 
The `Ecs` is a mapping of `Entity` to a set of components.
A `Query` is a set of components and is used to access entities from the world whos components intersect with the query. 

> **__Note:__** `Ecs` is usally called `World` outside of Feather. \
> We have chosen the name `Ecs` for the `World` to eliminate confusion with the Minecraft `World`.

An example
```rust
let world = game.world;
let entity = world.spawn((Player, Health { value: 10, max: 20}));
let query = world.query::<(&Player, &mut Health)>();
for (entity, (&player, &mut health) in query.iter() {
    health.value = 20;
}

for entity in query.iter() {
    // When `Ref<Player>` is dropped, `Player` is added back into the HList.
    let _: Ref<Player> = entity.get::<Player>();
    // When `Mut<Health>` is droped, `Health` is added back intot he HList.
    let health: Mut<Health> = entity.get_mut::<Health>();
    health.value = 20;
    // Fails since we already got a mutable reference (is removed from the HList).
    let health: Mut<Health> = entity.get_mut::<Health>();
}
```