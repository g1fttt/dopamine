#[repr(C)]
pub struct UserCommand {
    pad: [u8; 36],
    pub buttons: i32,
}

impl UserCommand {
    pub const IN_JUMP: i32 = 1 << 1;
}

#[repr(C)]
pub struct Entity {
    pad: [u8; 0x350],
    flags: i32,
}

impl Entity {
    const ON_GROUND: i32 = 1 << 0;

    #[inline(always)]
    pub fn is_on_ground(&self) -> bool {
        self.flags & Self::ON_GROUND != 0
    }
}
