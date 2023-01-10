struct Foo { // Total 16 Byte ORDERED ALIGNEMENT
    tiny: bool, // 1 Byte
    normal: u32, // 4 Byr
    small: u8, // 1 Byte
    long: u64, // 8 byte
    short: u16, // 2 Byte
} /*
long: u64, // 8 byte
normal: u32, // 4 Byte
short: u16, // 2 Byte
small: u8, // 1 Byte
tiny: bool, // 1 Byte
*/
#[repr(C)]
struct Foo2 { // C Alignement
    tiny: bool, // 1 Bit -> 1 Byte + 3 Byte Padding (because of normal u32)
    normal: u32, // 4 Byte 
    small: u8, // 1 Byte -> Already aligned (1+3+4). just add 1 byte for small
    long: u64, // 8 byte (1+3+4+1= 9 Byte), long (8 byte) must be aligned -> (1+3PAdding+4+1+7Padding+8) = 24 Byte
    short: u16, // 2 Byte ->ok 24+2 = 26 Bytes
} // Largest alignement of inner vars = u64 (8 byte). Foo must be aligned to 8 byte. From 26 to 32 Bytes

#[repr(packed)]
struct Foo3 { 
    tiny: bool, // 1 Byte
    normal: u32, // 4 Byte
    small: u8, // 1 Byte
    long: u64, // 8 byte
    short: u16, // 2 Byte
} 

#[repr(C)]
#[repr(align(16))] 
struct Foo4 { 
    
    tiny: bool, // 1 Byte
    normal: u32, // 4 Byte
    small: u8, // 1 Byte
    long: u64, // 8 byte
    short: u16, // 2 Byte
} 


fn main() {
    let a = Foo {
        tiny: true,
        normal: 1,
        small: 1,
        long: 1,
        short: 1,
    };

    println!("{} - {} - {} - {}",std::mem::size_of::<Foo>(),std::mem::size_of::<Foo2>(),std::mem::size_of::<Foo3>(),std::mem::size_of::<Foo4>());

}
