use Serializable;

pub enum Body {
    BodyRequest { FILESIZE: u64, FILENAME: Vec<u8> },
    BodyResponse { MSGID: u32, RESPONSE: u8 },
    BodyData { MSGID: u32, RESULT: u8 },
    BodyResult { MSGID: u32, RESULT: u8 }
}

impl Serializable for Body {
    fn get_bytes(&self) -> Vec<u8> {
        vec![1,2]
    }

    fn get_size(&self) -> u32 {
        16
    }
}

