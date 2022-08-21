/// 7zBuf.h/CBuf
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size)
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// 7zBuf.h/CDynBuf
struct DynamicBuffer {
    data: Vec<u8>,
    pos: usize,
}

impl DynamicBuffer {
    /// 7zBuf2.h/DynBuf_Construct
    pub fn construct() -> Self {
        Self {
            data: Vec::new(),
            pos: 0
        }
    }

    /// 7zBuf2.h/DynBuf_SeekToBeg
    pub fn seek_to_beginning(&mut self) {
        self.pos = 0;
    }

    /// 7zBuf2.h/DynBuf_Write
    pub fn write(&mut self, data: &Vec<u8>) {
        for v in data {
            self.data.push(*v)
        }
    }
}