use std::io::{self, Write, Read, Error, ErrorKind};
use message::Message;
use body::*;
use Serializable;
use header::Header;
use constant::*;

pub fn send<T: Write>(mut writer: T, message: Message) -> io::Result<usize> {
    let size = message.get_size() as usize;
    let mut bytes = vec![0; size];
    message.get_bytes(&mut bytes);
    writer.write(&bytes)
}

pub fn receive<T: Read>(mut reader: T) -> io::Result<Message> {
    let mut header_buffer: [u8; 16] = [0; 16];
    reader.read_exact(&mut header_buffer)?;
    let header = Header::new(&header_buffer);
    let mut body_buffer = vec![0; header.BODYLEN as usize];
    reader.read_exact(&mut body_buffer)?;
    let body = match header.MSGTYPE {
        REQ_FILE_SEND => Body::Request(BodyRequest::new(&body_buffer)),
        REP_FILE_SEND => Body::Response(BodyResponse::new(&body_buffer)),
        FILE_SEND_DATA => Body::Data(BodyData::new(&body_buffer)),
        FILE_SEND_RES => Body::Result(BodyResult::new(&body_buffer)),
        unknown => return Err(Error::new(ErrorKind::Other, format!("Unknown MSGTYPE : {}", unknown)))
    };
    Ok(Message::new(header, body))
}