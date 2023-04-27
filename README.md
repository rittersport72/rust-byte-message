# rust-byte-message
A message struct with byte layout is serialized to byte stream and vice versa.  
The byte stream is needed for transmission and reception in TcpStream or UdoSocket.

```
The byte layout (from right to left):
+----------+----------+----------+----------+  
|   Byte   |   Byte   |   Byte   |   Byte   |  
+==========+==========+==========+==========+  
|                  length                   |  
+----------+----------+----------+----------+  
|        spare        |        ident        |  
+----------+----------+----------+----------+  
|                   name                    |  
+----------+----------+----------+----------+  
|  state   |        unused       |   name   |  
+----------+----------+----------+----------+  
```

The above byte layout corresponds to this message struct:
```rust
#[repr(packed(1))]
pub struct ByteMessage {
    length: u32,               // 4 bytes
    ident: u16,                // 2 bytes
    spare: u16,                // 2 bytes
    name: [u8; 5],             // 5 bytes
    unused: u16,               // 2 bytes
    state: u8                  // 1 byte
}
```

Message struct encoding into array of u8:
```rust
// Create message struct
let message = ByteMessage::new();

// Encode message struct into byte stream
let array = message.to_bytes();    
```


Array of u8 decoding into message struct:
```rust
// Create message struct
let mut object = ByteMessage::new();

// Decode byte stream into message struct
object.from_bytes(&array);    
```

Only a fixed length array of u8 can be decoded. Convert slice of u8 into fixed length array:
```rust
// Create a fixed length array from a slice
let fixed_array = ByteMessage::array_of_byte_message(&slice[0..ByteMessage::MESSAGE_LENGTH]);
```
