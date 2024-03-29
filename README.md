# General
The code in this repository was used as a learning exercise to understand the architecture and layout of the NES. It was also used as a learning tool for my first larger program in Rust. As such, it is not optimized and likely contains anti-patterns that experienced Rust programmers would avoid. Furthermore, there are lots of additional refinements that need to be done for this to be a production ready program (documentation, multi-threading clean-up, mapper additions, and much more).

Note that none of the code in this repository is intended to be used for bypassing copyright protections. It is solely intended to be used as a way to learn about video game console architecture of the 8-bit era.

# Special Thanks
## OneLoneCoder
This program could not have been created without the help of OneLoneCoder's YouTube tutorial series on how the NES architecture works. One of the bigger challenges in doing this in Rust was finding a game engine and sound engine that could be used in Rust to do some of the things that OneLoneCoder gets for 'free' from his pixel game engine.

If you want to see his implementation, you can find it [here](https://github.com/OneLoneCoder/olcNES).

If you want to see his YouTube channel, you can find it [here](https://www.youtube.com/@javidx9).

## NES Dev Wiki
I'd also like to thank the folks at the [NES Dev Wiki](https://www.nesdev.org/wiki/Nesdev_Wiki) as it was instrumental in understanding some of the nuances and details of the system architecture. Specifically, implementing some of the illegal 6502 opcodes that are used by some games and the nestest.nes ROM which was used for verifying the 6502 implementation were indispensable to this project.

# License
This code is released under the provided LICENSE file. Note that OLC_LICENSE contains the original OneLoneCoder license, as required, since his code was referenced during the making of this program.
