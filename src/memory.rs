/// # Memory Map:
///
/// ```
/// +---------------+= 0xFFF (4095) End of Chip-8 RAM
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// | 0x200 to 0xFFF|
/// |     Chip-8    |
/// | Program / Data|
/// |     Space     |
/// |               |
/// |               |
/// |               |
/// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
/// |               |
/// |               |
/// |               |
/// +---------------+= 0x200 (512) Start of most Chip-8 programs
/// | 0x000 to 0x1FF|
/// | Reserved for  |
/// |  interpreter  |
/// +---------------+= 0x000 (0) Start of Chip-8 RAM
/// ```
#[derive(Debug)]
struct Memory {
    data: [u8; 4096],
}

impl Memory {
    fn new() -> Self {
        let mut data = [0; 4096];
        data[0..5] = [0xF0, 0x90, 0x90, 0x90, 0xF0]; // "0"
        data[5..10] = [0x20, 0x60, 0x20, 0x20, 0x70]; // "1"
        data[10..15] = [0xF0, 0x10, 0xF0, 0x80, 0xF0]; // "2"
        data[15..20] = [0xF0, 0x10, 0xF0, 0x10, 0xF0]; // "3"
        data[20..25] = [0x90, 0x90, 0xF0, 0x10, 0x10]; // "4"
        data[25..30] = [0xF0, 0x80, 0xF0, 0x10, 0xF0]; // "5"
        data[30..35] = [0xF0, 0x80, 0xF0, 0x90, 0xF0]; // "6"
        data[35..40] = [0xF0, 0x10, 0x20, 0x40, 0x40]; // "7"
        data[40..45] = [0xF0, 0x90, 0xF0, 0x90, 0xF0]; // "8"
        data[45..50] = [0xF0, 0x90, 0xF0, 0x10, 0xF0]; // "9"
        data[50..55] = [0xF0, 0x90, 0xF0, 0x90, 0x90]; // "A"
        data[55..60] = [0xE0, 0x90, 0xE0, 0x90, 0xE0]; // "B"
        data[60..65] = [0xF0, 0x80, 0x80, 0x80, 0xF0]; // "C"
        data[65..70] = [0xE0, 0x90, 0x90, 0x90, 0xE0]; // "D"
        data[70..75] = [0xF0, 0x80, 0xF0, 0x80, 0xF0]; // "E"
        data[75..80] = [0xF0, 0x80, 0xF0, 0x80, 0x80]; // "F"
        Self { data };
    }

    fn address(&self, addr: usize) -> Option<&u8> {
        if addr >= 0x200 && addr <= 0x1000 {
            Some(&self.data[addr])
        } else {
            None
        }
    }

    fn assign(&mut self, addr: usize, value: u8) -> Result<(), String> {
        if addr >= 0x200 && addr <= 0x1000 {
            self.data[addr] = value;
            Ok(())
        } else {
            Err(format!("Invalid memory address: 0x{:X}", addr))
        }
    }
}
