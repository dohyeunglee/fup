pub const REQ_FILE_SEND: u32 = 0x01;
pub const REP_FILE_SEND: u32 = 0x02;
pub const FILE_SEND_DATA: u32 = 0x03;
pub const FILE_SEND_RES: u32 = 0x04;

pub const NOT_FRAGMENTED: u8 = 0x00;
pub const FRAGMENTED: u8 = 0x01;

pub const NOT_LASTMSG: u8 = 0x00;
pub const LASTMSG: u8 = 0x01;

pub const ACCEPTED: u8 = 0x00;
pub const DENIED: u8= 0x01;

pub const FAIL: u8 = 0x00;
pub const SUCCESS: u8 = 0x01;
