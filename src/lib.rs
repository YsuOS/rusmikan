#![no_std]

pub enum PixelFormat {
    RGB,
    BGR,
}

pub struct FrameBuffer {
    pub base: *mut u8,
}

impl FrameBuffer {
    pub unsafe fn write_value(&mut self, index: usize, val: [u8;3]) {
        (self.base.add(index) as *mut [u8;3]).write_volatile(val)
    }
}

pub struct FrameBufferConfig {
    pub frame_buffer: FrameBuffer,
    pub pixels_per_scan_line: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
}
