# Number Deconstruction
I couldn't think of a better name for it.
Turns numbers into slices of other number types. Originally made to turn `u64`/`u32`/`u16` into `u8`s to write to file and back.
Note: this returns slices, like `[u8; 2]` instead of tuples. No real reason, beyond slices having more useful methods.

Unsure if I should have made them inline, but I imagine the compiler will handle that.
Has:
`u16` -> 2 `u8`
2 `u8` -> `u16`

`u32` -> 2 `u16`
`u32` -> 4 `u8`
2 `u16` -> `u32`
4 `u8` -> `u32`

`u64` -> 2 `u32`
`u64` -> 4 `u16`
`u64` -> 8 `u8`
2 `u32` -> `u64`
4 `u16` -> `u64`
8 `u8` -> `u64`
