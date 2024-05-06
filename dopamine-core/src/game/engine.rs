use dopamine_proc_macro::virtual_method;

#[repr(C)]
pub struct Engine;

impl Engine {
    #[virtual_method(index = 12)]
    fn local_player_index(&self) -> i32;
}
