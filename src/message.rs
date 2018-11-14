use header::Header;
use body::Body;
use Serializable;

pub struct Message {
    pub header: Header,
    pub body: Body
}

impl Message {
    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_body(&self) -> &Body {
        &self.body
    }

}

impl Serializable for Message {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0; self.get_size() as usize];
        let header_size = self.header.get_size() as usize;
        bytes[0..header_size].copy_from_slice(&self.header.get_bytes());
        bytes[header_size..].copy_from_slice(&self.body.get_bytes());
        bytes
    }


    fn get_size(&self) -> u32 {
        self.header.get_size() + self.body.get_size()
    }

}

