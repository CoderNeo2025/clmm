pub mod tick;
pub use tick::*;

use anchor_lang::prelude::*;

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PoolState {
    pub bump: u8,
    pub tick_spacing: u16,
    pub tick_current: i32,
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub fee_rate: u32, // units of hundredths of a basis point(0.0001%)
    pub protocol_fee_ratio: u32,

    pub liquidity: u128,
    /// sqrt(token1/token0), Q64.64 value
    pub sqrt_price_x64: u128,
    pub fee_growth_global0_x64: u128,
    pub fee_growth_global1_x64: u128,
    pub protocol_fees0: u128,
    pub protocol_fees1: u128,
}

impl PoolState {
    pub const LEN: usize =
        1 +
        2 +
        4 +
        32 +
        32 +
        4 +
        4 +
        16 +
        16 +
        16 +
        16 +
        16 +
        16;
}

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