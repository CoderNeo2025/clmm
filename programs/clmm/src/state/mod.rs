#[macro_use]
pub mod tick;
pub use tick::*;
pub use tick_index_check;

pub mod pool;
pub use pool::*;

use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct PositionState {
    pub liquidity: u128,
    pub fee_growth_inside_0_last_x64: u128,
    pub fee_growth_inside_1_last_x64: u128,
}

impl PositionState {
    pub const LEN: usize =
        16 +
        16 +
        16;

    pub fn initialize(&mut self, l_delta: u128, fee_g_0: u128, fee_g_1: u128) {
        self.liquidity = l_delta;
        self.fee_growth_inside_0_last_x64 = fee_g_0;
        self.fee_growth_inside_1_last_x64 = fee_g_1;
    }
}