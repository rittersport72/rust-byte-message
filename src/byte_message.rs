#![allow(dead_code)]
use std::mem;

// Message layout of network packet
// +----------+----------+----------+----------+  +-------------+
// |   Byte   |   Byte   |   Byte   |   Byte   |  |    Bytes    |
// +==========+==========+==========+==========+  +=============+
// |                  length                   |  |  3  2  1  0 |
// +----------+----------+----------+----------+  +-------------+
// |        spare        |        ident        |  |  7  6  5  4 |
// +----------+----------+----------+----------+  +-------------+
// |                   name                    |  | 11 10  9  8 |
// +----------+----------+----------+----------+  +-------------+
// |  state   |        unused       |   name   |  | 15 14 13 12 |
// +----------+----------+----------+----------+  +-------------+
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
//#[derive(Debug, PartialEq)]
pub struct ByteMessage {
    length: u32,               // 4 bytes
    ident: u16,                // 2 bytes
    spare: u16,                // 2 bytes
    name: [u8; 5],             // 5 bytes
    unused: u16,               // 2 bytes
    state: u8                  // 1 byte
}

/*
* Implementation ByteMessage
*/
impl ByteMessage {
    pub fn new() -> Self {
        ByteMessage {
            length: ByteMessage::MESSAGE_LENGTH as u32,
            ident: 0,
            spare: 0,
            name: [0; 5],
            unused: 0,
            state: 0
        }
    }
    
    /*
    * Convert byte stream to struct. This uses unsafe.
    */
    pub fn from_bytes(&mut self, array: &[u8; ByteMessage::MESSAGE_LENGTH]) {
        unsafe { *self = mem::transmute_copy::<[u8; ByteMessage::MESSAGE_LENGTH], ByteMessage>(array); }
    }
    
    /*
    * Convert struct to byte stream. This uses unsafe.
    */
    pub fn to_bytes(&self) -> [u8; ByteMessage::MESSAGE_LENGTH] {
        unsafe { mem::transmute_copy::<ByteMessage, [u8; ByteMessage::MESSAGE_LENGTH]>(self) }
    }
    
    /*
    * Create fixed length array from slice.
    */
    fn array_of_byte_message(array: &[u8]) -> [u8; ByteMessage::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }
    
    /*
    * Set length (host byte order).
    */
    pub fn set_length(&mut self, length: u32) {
        self.length = length.to_be();
    }
    
    /*
    * Get length (host byte order).
    */
    pub fn get_length(&self) -> u32 {
        return u32::from_be(self.length);
    }
    
    /*
    * Set ident (host byte order).
    */
    pub fn set_ident(&mut self, ident: u16) {
        self.ident = ident.to_be();
    }
    
    /*
    * Get ident (host byte order).
    */
    pub fn get_ident(&self) -> u16 {
        return u16::from_be(self.ident);
    }
    
    /*
    * Set name.
    */
    pub fn set_name(&mut self, name: [u8; 5]) {
        self.name = name;
    }
    
    /*
    * Get name.
    */
    pub fn get_name(&self) -> [u8; 5] {
        return self.name;
    }
    
    /*
    * Set state.
    */
    pub fn set_state(&mut self, state: u8) {
        self.state = state.to_be();
    }
    
    /*
    * Get state.
    */
    pub fn get_state(&self) -> u8 {
        return u8::from_be(self.state);
    }
    
    /*
    * Message length in memory.
    */
    pub const MESSAGE_LENGTH : usize = mem::size_of::<Self>();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn message() {
        let name: [u8; 5] = [ 30, 35, 40, 45, 50 ]; 
        
        let mut message = ByteMessage::new();
        message.set_length(123);
        message.set_ident(26);
        message.set_name(name);
        message.set_state(42);
        
        assert_eq!(message.get_length(), 123);
        assert_eq!(message.get_ident(), 26);
        assert_eq!(message.get_name(), name);
        assert_eq!(message.get_state(), 42);
    }
}