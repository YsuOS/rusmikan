#![no_std]

use uefi::proto::console::gop::FrameBuffer;

pub enum PixelFormat {
    PixelRGBResv8BitPerColor,
    PixelBGRResv8BitPerColor,
}

pub struct FrameBufferConfig<'gop> {
    pub frame_buffer: FrameBuffer<'gop>,
    pub pixels_per_scan_line: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
}
