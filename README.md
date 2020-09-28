# Chip8Emulator
Chip 8 emulator written in rust

Things that helped me:
- [Cowgods Chip8 guide](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Fx0A)
- [starrhorne's interpreter](https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/input_driver.rs)
- [AlexEne's interpreter](https://github.com/AlexEne/rust-chip8/blob/main/src/cpu.rs)
- https://github.com/chip8-rust/chip8-ui/blob/master/
# Design

Running a program:
1. filename is passed as arg to program
2. load program into memory
3. Start executing instructions
