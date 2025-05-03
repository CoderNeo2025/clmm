#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod libraries;
pub mod util;

use anchor_lang::prelude::*;
use instructions::*;
declare_id!("FAsGDFLK4uPpSuJPJYzXx6iWR3f3w6hvtcXCsVX5maS5");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, 
                           tick_spacing: u16,sqrt_price_x64: u128,
                           fee_ratio: u32, protocol_fee: u32) -> Result<()> {
        instructions::initialize_pool_impl(ctx, tick_spacing, sqrt_price_x64, fee_ratio, protocol_fee)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    ) -> Result<()> {
        open_position_impl(ctx, tick_lower, tick_upper, liquidity, amount_0_max, amount_1_max)
    }
}
