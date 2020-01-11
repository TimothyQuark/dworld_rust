# dworld_rust

A project to learn Rust, which follows the Roguelike Rust tutorial: http://bfnightly.bracketproductions.com/rustbook/chapter_0.html

# dworld_rust

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.