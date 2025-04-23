#[macro_use]
pub mod tick;
pub use tick::*;
pub use tick_index_check;

pub mod pool;
pub use pool::*;

use anchor_lang::prelude::*;

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PositionState {
    pub liquidity: u128,
    pub fee_growth_inside0_last_x64: u128,
    pub fee_growth_inside1_last_x64: u128,
}

impl PositionState {
    pub const LEN: usize =
        16 +
        16 +
        16;
}