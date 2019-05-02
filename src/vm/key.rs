#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Key {
    Key0 = 0x0, Key1 = 0x1, Key2 = 0x2, Key3 = 0x3,
    Key4 = 0x4, Key5 = 0x5, Key6 = 0x6, Key7 = 0x7,
    Key8 = 0x8, Key9 = 0x9, KeyA = 0xa, KeyB = 0xb,
    KeyC = 0xc, KeyD = 0xd, KeyE = 0xe, KeyF = 0xf,
}

impl Key {
    /// +---+---+---+---+    +---+---+---+---+
    /// | 1 | 2 | 3 | 4 |    | 1 | 2 | 3 | C |
    /// +---+---+---+---+ => +---+---+---+---+
    /// | Q | W | E | R |    | 4 | 5 | 6 | D |
    /// +---+---+---+---+ => +---+---+---+---+
    /// | A | S | D | F |    | 7 | 8 | 9 | E |
    /// +---+---+---+---+ => +---+---+---+---+
    /// | Z | X | C | V |    | A | 0 | B | F |
    /// +---+---+---+---+    +---+---+---+---+
    pub fn from(key: minifb::Key) -> Option<Self> {
        match key {
            minifb::Key::Key1 => Some(Key::Key1),
            minifb::Key::Key2 => Some(Key::Key2),
            minifb::Key::Key3 => Some(Key::Key3),
            minifb::Key::Key4 => Some(Key::KeyC),

            minifb::Key::Q => Some(Key::Key4),
            minifb::Key::W => Some(Key::Key5),
            minifb::Key::E => Some(Key::Key6),
            minifb::Key::R => Some(Key::KeyD),

            minifb::Key::A => Some(Key::Key7),
            minifb::Key::S => Some(Key::Key8),
            minifb::Key::D => Some(Key::Key9),
            minifb::Key::F => Some(Key::KeyE),

            minifb::Key::Z => Some(Key::KeyA),
            minifb::Key::X => Some(Key::Key0),
            minifb::Key::C => Some(Key::KeyB),
            minifb::Key::V => Some(Key::KeyF),

            _ => None,
        }
    }
}
