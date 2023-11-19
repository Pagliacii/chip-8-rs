/// # Display
///
/// ## Modes
///
/// - 64x32
/// - 64x48
/// - 64x64
/// - 128x64
///
/// ## Sprites
///
/// Chip-8 draws graphics on screen through the use of sprites. A sprite is a
/// group of bytes which are a binary representation of the desired picture.
/// Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.
///
/// Programs may also refer to a group of sprites representing the hexadecimal
/// digits 0 through F. These sprites are 5 bytes long, or 8x5 pixels. The data
/// should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF).
