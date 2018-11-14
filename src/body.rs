use Serializable;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use std::mem;

pub enum Body {
    Request(BodyRequest),
    Response(BodyResponse),
    Data(BodyData),
    Result(BodyResult)
}

pub struct BodyRequest {
    pub FILESIZE: u64,
    pub FILENAME: Vec<u8>
}

pub struct BodyResponse {
    pub MSGID: u32,
    pub RESPONSE: u8
}

pub struct BodyData {
    pub DATA: Vec<u8>
}

pub struct BodyResult {
    pub MSGID: u32,
    pub RESULT: u8
}

impl BodyRequest {
    fn new(bytes: &[u8]) -> BodyRequest {
        let size_of_long = mem::size_of::<u64>();
        let mut file_name_bytes = vec![0; bytes.len() - size_of_long];
        file_name_bytes.copy_from_slice(&bytes[size_of_long..]);
        BodyRequest {
            FILESIZE: (&bytes[..size_of_long]).read_u64::<NativeEndian>().unwrap(),
            FILENAME: file_name_bytes
        }
    }
}

impl Serializable for BodyRequest {
    fn get_bytes(&self, bytes: &mut [u8]) {
        let size_of_long = mem::size_of::<u64>();
        (&mut bytes[..size_of_long]).write_u64::<NativeEndian>(self.FILESIZE).unwrap();
        bytes[size_of_long..].copy_from_slice(&self.FILENAME);
    }

    fn get_size(&self) -> u32 {
        (mem::size_of::<u64>() + self.FILENAME.len()) as u32
    }
}

impl Serializable for Body {
    fn get_bytes(&self, bytes: &mut [u8]) { }

    fn get_size(&self) -> u32 {
        12
//        match self {
//            Body::Request(body_request) => body_request.get_size(),
//            Body::Response(body_response) => body_response.get_size(),
//            Body::Data(body_data) => body_data.get_size(),
//            Body::Result(body_result) => body_result.get_size(),
//        }
    }
}


