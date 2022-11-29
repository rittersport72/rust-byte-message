pub mod byte_message;

use byte_message::ByteMessage;

/*
* Use byte message with network layout.
*/
fn main() {
    let name: [u8; 5] = [ 30, 35, 40, 45, 50 ]; 
    
    // Original message
    let mut message = ByteMessage::new();
    message.set_ident(26);
    message.set_name(name);
    message.set_state(42);

    // Convert struct to byte stream
    let array = message.to_bytes();
    
    // New message
    let mut object = ByteMessage::new();
    // Convert byte stream to struct
    object.from_bytes(&array);
    
    assert_eq!(message.get_length(), object.get_length());
    assert_eq!(message.get_ident(), object.get_ident());
    assert_eq!(message.get_name(), object.get_name());
    assert_eq!(message.get_state(), object.get_state());
}