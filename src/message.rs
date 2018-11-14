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
    fn get_bytes(&self, bytes: &mut [u8]) {
        let header_size = self.header.get_size() as usize;
        self.header.get_bytes(&mut bytes[0..header_size]);
        self.body.get_bytes(&mut bytes[header_size..]);
    }


    fn get_size(&self) -> u32 {
        self.header.get_size() + self.body.get_size()
    }

}

