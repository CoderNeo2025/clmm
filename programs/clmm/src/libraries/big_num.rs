use uint::construct_uint;

macro_rules! impl_big_num {
    ($t: ty, $sz: expr) => {
        impl anchor_lang::Space for $t {
            const INIT_SPACE: usize = $sz;
        }

        unsafe impl bytemuck::Zeroable for $t {}

        unsafe impl bytemuck::Pod for $t {}
    }
}

construct_uint!{
    pub struct U128(2);
}

impl_big_num!(U128, 16);

construct_uint!{
    pub struct U256(4);
}

impl_big_num!(U256, 32);

construct_uint!{
    pub struct U512(8);
}

impl_big_num!(U512, 64);

construct_uint!{
    pub struct U1024(16);
}

impl_big_num!(U1024, 128);
