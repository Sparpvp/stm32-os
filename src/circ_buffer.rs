pub struct CircularBuffer {
    buf: [u8; 40],
    read_index: u8,
    write_index: u8,
}

pub static mut G_BUFFER: Option<CircularBuffer> = None;

impl CircularBuffer {
    pub fn init() {
        assert_eq!(unsafe { G_BUFFER.is_none() }, true);

        let cb = CircularBuffer {
            buf: [0; 40],
            read_index: 0,
            write_index: 0,
        };
        unsafe {
            G_BUFFER.replace(cb);
        };
    }

    // TODO: put, get
}
