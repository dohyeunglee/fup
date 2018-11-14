extern crate byteorder;

mod constant;
mod message;
mod header;
mod body;

pub trait Serializable {
    fn get_bytes(&self, bytes:&mut [u8]);
    fn get_size(&self) -> u32;
}

