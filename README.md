# Chip8Emulator
Chip 8 emulator I used as both an introduction to Rust and writting emulators!

![alt text](SpaceInvaders.png)

## Usage
1. Install SDL2 (Can be done on MacOS using Homebrew `brew install sdl2`)
2. Install binary using cargo
    ```
    cargo install chip8-rs
    ```
3. Download a chip8 rom and play it!
    ```
    chip8-rs <path/to/rom>
    ```

## TODO
Beeping


# References
Things that helped me:
- [Cowgods Chip8 guide](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Fx0A)
- [starrhorne's interpreter](https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/input_driver.rs)
- [AlexEne's interpreter](https://github.com/AlexEne/rust-chip8/blob/main/src/cpu.rs)
- https://github.com/chriskonstad/chip8/blob/master/src/lib.rs
- https://github.com/chip8-rust/chip8-ui/blob/master/
