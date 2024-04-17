
use byteorder::{ByteOrder, BigEndian};

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }



    pub fn read_f4(&mut self) -> f32 {
        let result = BigEndian::read_f32(&self.data[self.pos..self.pos+4]);
        self.pos += 4;
        result
    }


    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let result = self.data[self.pos..self.pos+length].to_vec();
        self.pos += length;
        result
    }

    pub fn read_u1(&mut self) -> u8 {
        let result = self.data[self.pos];
        self.pos += 1;
        result
    }

    pub fn read_u2(&mut self) -> u16 {
        let result = BigEndian::read_u16(&self.data[self.pos..self.pos+2]);
        self.pos += 2;
        result
    }

    pub fn read_u4(&mut self) -> u32 {
        let result = BigEndian::read_u32(&self.data[self.pos..self.pos+4]);
        self.pos += 4;
        result
    }

    pub fn read_u8(&mut self) -> u64 {
        let result = BigEndian::read_u64(&self.data[self.pos..self.pos+8]);
        self.pos += 8;
        result
    }



}