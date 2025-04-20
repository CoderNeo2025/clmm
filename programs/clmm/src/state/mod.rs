pub mod tick;
pub use tick::*;

use anchor_lang::prelude::*;
use crate::libraries::big_num::U256;

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(InitSpace, Default, Debug)]
pub struct PoolState {
    pub bump: u8,
    pub tick_spacing: u16,
    pub tick_current: i32,
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub fee_rate: u32, // units of hundredths of a basis point(0.0001%)
    pub protocol_fee_ratio: u32,

    pub liquidity: u128,
    pub sqrt_price: u128,
    pub fee_growth_global0: U256,
    pub fee_growth_global1: U256,
    pub protocol_fees0: u128,
    pub protocol_fees1: u128,
}

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(InitSpace, Default, Debug)]
pub struct PositionState {
    pub liquidity: u128,
    pub fee_growth_inside0_last: U256,
    pub fee_growth_inside1_last: U256,
}