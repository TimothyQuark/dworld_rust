# dworld_rust

A project to learn Rust, which follows the Roguelike Rust tutorial: http://bfnightly.bracketproductions.com/rustbook/chapter_0.html

# How to run

Windows:
- Go to releases and download dworld-windows.
- On the Github page, open the resources folder, and download the cp437_20x20.png spritesheet.
- On your PC, create a folder called resources next to the executable. Place the spritesheet inside.
- The executable should run by double clicking it. Note that Windows Defender may warn you of an unknown program. If you feel unsafe opening such a file, you can build the game from source instead.

Linux:
- Go to releases and download dworld-linux.
- On the Github page, open the resources folder, and download the cp437_20x20.png spritesheet.
- On your PC, create a folder called resources next to the executable. Place the spritesheet inside.
- The executable should run by double clicking it.
- If for some reason your File Manager does not recognize the game as an executable, you may have to run it through the terminal.
- This can be done by opening the Terminal in the folder where the executable is, and then running ./name-of-program, ex. ~$ ./dworld-linux

MacOS
- Unsure, probably very similar to the Linux instructions.

Build from Source:
- Install Rust on your computer (rustup is preferred toolchain installer)
- Download or clone the repo to a location on your computer (such as Documents): git clone https://github.com/TimothyQuark/dworld_rust
- If using rustup, game can simply be built by opening the program directory in terminal, and running "cargo build"
- Play the game with "cargo run", or move the executable out of the target/debug folder (note you also need to move the spritesheet with it)