#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod libraries;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FAsGDFLK4uPpSuJPJYzXx6iWR3f3w6hvtcXCsVX5maS5");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, 
                           tick_spacing: u16,sqrt_price_x64: u128,
                           fee_ratio: u32, protocol_fee: u32) -> Result<()> {
        instructions::initialize_pool_impl(ctx, tick_spacing, sqrt_price_x64, fee_ratio, protocol_fee)
    }

    pub fn add_liquidity(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn remove_liquidity(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn swap(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn collect_fees(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_protocol_parameters(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn create_position(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_tick(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_oracle(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }
}
