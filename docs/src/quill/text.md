# Text
Minecraft uses [Raw JSON text format](https://minecraft.gamepedia.com/Raw_JSON_text_format) which is not very convinient to write by hand. Quill provides a builder abstraction through [`Text`]().

[`Text::of`]() instantiate an unformatted builder with the given string.
```rust
Text::of("hello world");
```

Adding style, color, and actions to the `Text` builder.
```rust
Text::of("red bold text")
    .red()
    .bold()
    .on_click_open_url("https://feathermc.org");
```

You can conact multiple builders together via [`Text::concat`]().
```rust
Text::of("red bold text")
    .red()
    .bold()
    .concat(
        Text::of("blue italic text")
            .blue()
            .italic()
    );
```

There is also the `Text::translate` builder, which is inteperated on the client side and replaces.
```rust
Text::translate("Hi %s", vec![username])
```