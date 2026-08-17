/// Returns ST's 12-byte device electronic signature with 4 bytes 0x4C padding
/// as a 128-bit UUID for use with e.g. Cyphal.
pub fn uuid() -> [u8; 16] {
    const UID_ADDRESS: u32 = 0x1FFF_7590;

    let mut uuid: [u8; 16] = [0x4C; 16];
    // SAFETY: 😊
    unsafe {
        core::ptr::copy_nonoverlapping(UID_ADDRESS as *const u8, uuid.as_mut_ptr(), 12);
    }

    uuid
}
