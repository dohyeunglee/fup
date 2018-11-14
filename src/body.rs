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

impl BodyRequest {
    pub fn new(bytes: &[u8]) -> BodyRequest {
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

pub struct BodyResponse {
    pub MSGID: u32,
    pub RESPONSE: u8
}

impl BodyResponse {
    pub fn new(bytes: &[u8]) -> BodyResponse {
        let size_of_uint = mem::size_of::<u32>();
        BodyResponse {
            MSGID: (&bytes[..size_of_uint]).read_u32::<NativeEndian>().unwrap(),
            RESPONSE: bytes[size_of_uint]
        }
    }
}

impl Serializable for BodyResponse {
    fn get_bytes(&self, bytes: &mut [u8]) {
        let size_of_uint = mem::size_of::<u32>();
        (&mut bytes[..size_of_uint]).write_u32::<NativeEndian>(self.MSGID).unwrap();
        bytes[size_of_uint] = self.RESPONSE;
    }

    fn get_size(&self) -> u32 {
        (mem::size_of::<u32>() + mem::size_of::<u8>()) as u32
    }
}

pub struct BodyData {
    pub DATA: Vec<u8>
}

impl BodyData {
    pub fn new(bytes: &[u8]) -> BodyData {
        BodyData {
            DATA: bytes.to_vec()
        }
    }
}

impl Serializable for BodyData {
    fn get_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.DATA);
    }

    fn get_size(&self) -> u32 {
        self.DATA.len() as u32
    }
}

pub struct BodyResult {
    pub MSGID: u32,
    pub RESULT: u8
}

impl BodyResult {
    pub fn new(bytes: &[u8]) -> BodyResult {
        let size_of_uint = mem::size_of::<u32>();
        BodyResult {
            MSGID: (&bytes[..size_of_uint]).read_u32::<NativeEndian>().unwrap(),
            RESULT: bytes[size_of_uint]
        }
    }
}

impl Serializable for BodyResult {
    fn get_bytes(&self, bytes: &mut [u8]) {
        let size_of_uint = mem::size_of::<u32>();
        (&mut bytes[..size_of_uint]).write_u32::<NativeEndian>(self.MSGID).unwrap();
        bytes[size_of_uint] = self.RESULT;
    }

    fn get_size(&self) -> u32 {
        (mem::size_of::<u32>() + mem::size_of::<u8>()) as u32
    }
}


impl Serializable for Body {
    fn get_bytes(&self, bytes: &mut [u8]) {
        match self {
            Body::Request(body_request) => body_request.get_bytes(bytes),
            Body::Response(body_response) => body_response.get_bytes(bytes),
            Body::Data(body_data) => body_data.get_bytes(bytes),
            Body::Result(body_result) => body_result.get_bytes(bytes),
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            Body::Request(body_request) => body_request.get_size(),
            Body::Response(body_response) => body_response.get_size(),
            Body::Data(body_data) => body_data.get_size(),
            Body::Result(body_result) => body_result.get_size(),
        }
    }
}


