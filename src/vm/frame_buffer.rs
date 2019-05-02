const ON: u32 = u32::max_value();
const OFF: u32 = u32::min_value();

pub struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![OFF; width * height],
            width,
            height,
        }
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) -> bool {
        let l = y * self.width + x;
        assert!(l < self.width * self.height);

        if self.buffer[l] == ON {
            self.buffer[l] = OFF;
            true
        } else {
            self.buffer[l] = ON;
            false
        }
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn clear(&mut self) {
        for i in 0..(self.width * self.height) {
            self.buffer[i] = 0x0;
        }
    }
}
