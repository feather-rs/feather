# Commands
Quill uses a builders to define commands, it leverages the type system of rust.

- `literal(value: &str)`
- `any()`
- `option()`

- `CommandBuilder::arg::<T>() where T: ArgumentParser`
- `CommandBuilder::opt::<T>() where T: ArgumentParser`
- `CommandBuilder::and::<T>() where T: CommandBuilder`
- `CommandBuilder::literal(value: &str)`

Any type which implements argument parser, can be parsed and used via the 
`ArgumentParser`

```rust
let tp = literal("tp")
    .arg()
    .arg()
    .arg()
    .build(|x: i32, y: i32, z: i32| move |game: Game| {
        ...
    });
```

You can also define overlapping commands.
```rust
let tp_sender = literal("tp")
    .arg()
    .arg()
    .arg()
    .build(|x: i32, y: i32, z: i32| move |game: Game| {
        ...
    });

let tp_target = literal("tp")
    .and(target())
    .arg()
    .arg()
    .arg()
    .build(|target: Player, x: i32, y: i32, z: i32| move |game: Game| {
        ...
    });

let tp = tp_sender.or(tp_target);
```

You can also define optional arguments.
```rust
let tp_target = literal("tp")
    .opt()
    .arg()
    .arg()
    .arg()
    .build(|target: Option<Player>, x: i32, y: i32, z: i32| move |game: Game| {
        ...
    });
```

You can register built commands via `Game::register_command`
```rust
game.register_command(tp);
```