use Serializable;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone)]
pub struct Header {
    pub MSGID: u32,
    pub MSGTYPE: u32,
    pub BODYLEN: u32,
    pub FRAGMENTED: u8,
    pub LASTMSG: u8,
    pub SEQ: u16
}

impl Header {
    pub fn new(bytes: &[u8]) -> Header {
        Header {
            MSGID: (&bytes[0..4]).read_u32::<NativeEndian>().unwrap(),
            MSGTYPE: (&bytes[4..8]).read_u32::<NativeEndian>().unwrap(),
            BODYLEN: (&bytes[8..12]).read_u32::<NativeEndian>().unwrap(),
            FRAGMENTED: bytes[12],
            LASTMSG: bytes[13],
            SEQ: (&bytes[14..16]).read_u16::<NativeEndian>().unwrap()
        }
    }
}

impl Serializable for Header {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes: [u8; 16] = [0; 16];
        (&mut bytes[..4]).write_u32::<NativeEndian>(self.MSGID).unwrap();
        (&mut bytes[4..8]).write_u32::<NativeEndian>(self.MSGTYPE).unwrap();
        (&mut bytes[8..12]).write_u32::<NativeEndian>(self.BODYLEN).unwrap();
        bytes[12] = self.FRAGMENTED;
        bytes[13] = self.LASTMSG;
        (&mut bytes[14..]).write_u16::<NativeEndian>(self.SEQ).unwrap();
        bytes.to_vec()
    }

    fn get_size(&self) -> u32 {
        16
    }
}


